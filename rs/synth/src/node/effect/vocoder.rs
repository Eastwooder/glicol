use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use std::f32::consts::PI;

#[derive(Debug, Clone)]

pub struct PhaseVocoder {
    pitch_shift: f32,
    n: usize,
    hop_size: usize,
    fft_size: usize,
    prev_phase: Vec<f32>,
    omega: Vec<f32>,
    window: Vec<f32>,
    buffer: Vec<Complex<f32>>,
    output_buffer: Vec<Complex<f32>>,
    last_frame: Vec<Complex<f32>>,
    input_order: Vec<usize>
}

impl PhaseVocoder {
    pub fn new(pitch_shift: f32, time_stretch: f32) -> Self {
        let n = 128;
        let hop_size = (n as f32 / time_stretch) as usize;
        let fft_size = 2 * n;
        let prev_phase = vec![0.0; fft_size / 2 + 1];
        let omega = (0..fft_size / 2 + 1).map(|i| 2.0 * PI * i as f32 / fft_size as f32).collect();
        let window = (0..n).map(|i| 0.5 - 0.5 * (2.0 * PI * i as f32 / (n - 1) as f32).cos()).collect();
        let buffer = vec![Complex::zero(); fft_size];
        let output_buffer = vec![Complex::zero(); fft_size];
        let last_frame = vec![Complex::zero(); n];
        Self {
            pitch_shift,
            n,
            hop_size,
            fft_size,
            prev_phase,
            omega,
            window,
            buffer,
            output_buffer,
            last_frame,
            input_order: vec![]
        }
    }
    impl_to_boxed_nodedata!();

}

impl<const N:usize> Node<N> for PhaseVocoder {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        let input = &inputs.values_mut().next().unwrap().buffers()[0];

        // let mut out = vec![0.0; N * 2 / self.hop_size];
        for i in 0..(N / self.hop_size) {
            // copy input to buffer
            for j in 0..N {
                let idx = j + i * self.hop_size;
                if idx < N {
                    self.buffer[j] = Complex::new(input[idx] * self.window[j], 0.0);
                } else {
                    self.buffer[j] = Complex::zero();
                }
            }
            let mut planner = FftPlanner::new(); // false, rustfft::FftDirection::Forward
            let fft = planner.plan_fft(self.fft_size, rustfft::FftDirection::Forward);
            fft.process(&mut self.buffer);

            // calculate magnitudes and phases
            let mut magnitudes = vec![0.0; self.fft_size / 2 + 1];
            let mut phases = vec![0.0; self.fft_size / 2 + 1];
            for j in 0..(self.fft_size / 2 + 1) {
                let re = self.buffer[j].re;
                let im = self.buffer[j].im;
                magnitudes[j] = (re * re + im * im).sqrt();
                let phase = if re == 0.0 && im == 0.0 {
                    0.0
                } else {
                    (im / re).atan()
                };
                phases[j] = phase - self.prev_phase[j] - self.omega[j] * self.hop_size as f32 / self.fft_size as f32;
                self.prev_phase[j] = phase;
            }
            // apply phase vocoder
            let ratio = 2.0_f32.powf(self.pitch_shift / 12.0 / self.hop_size as f32);
            for j in 0..(self.fft_size / 2 + 1) {
                let magnitude = magnitudes[j];
                let phase = phases[j];
                let new_phase = phase + self.omega[j] * self.hop_size as f32 / self.fft_size as f32 * ratio;
                self.output_buffer[j] = Complex::new(new_phase.cos() * magnitude, new_phase.sin() * magnitude);
            }
            // apply inverse FFT
            let mut planner = FftPlanner::new();
            let ifft = planner.plan_fft(self.fft_size, rustfft::FftDirection::Inverse);
            ifft.process(&mut self.output_buffer);
            // copy output to output buffer, overlap-add with previous frame
            for j in 0..N {
                let idx = j + i * self.hop_size;
                if idx < input.len() {
                    output[0][idx] += self.output_buffer[j].re * self.window[j] + self.last_frame[j].im * self.window[j];
                }
                self.last_frame[j] = self.output_buffer[j];
            }

        }
        // for i in 0..N {
        //     // output[0][i] = 0.0;
        // }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, _v) => {
                match pos {
                    // 0 => {self.pitch_shift = value},
                    // 1 => {self.time_stretch = value},
                    _ => {}
                }
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}