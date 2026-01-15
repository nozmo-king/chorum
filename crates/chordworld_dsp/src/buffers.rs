//! Audio buffer and bus abstractions

use smallvec::SmallVec;

/// Maximum channels to store inline (before heap allocation)
const MAX_INLINE_CHANNELS: usize = 8;

/// Audio bus reference (read-only slices for each channel)
pub struct AudioBusRef<'a> {
    channels: SmallVec<[&'a [f32]; MAX_INLINE_CHANNELS]>,
}

impl<'a> AudioBusRef<'a> {
    pub fn new(channels: impl IntoIterator<Item = &'a [f32]>) -> Self {
        Self {
            channels: channels.into_iter().collect(),
        }
    }

    pub fn from_slice(data: &'a [f32]) -> Self {
        Self::new(std::iter::once(data))
    }

    pub fn channel(&self, index: usize) -> Option<&[f32]> {
        self.channels.get(index).copied()
    }

    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
    }
}

/// Audio bus mutable reference (writable slices for each channel)
pub struct AudioBusMut<'a> {
    channels: SmallVec<[&'a mut [f32]; MAX_INLINE_CHANNELS]>,
}

impl<'a> AudioBusMut<'a> {
    pub fn new(channels: impl IntoIterator<Item = &'a mut [f32]>) -> Self {
        Self {
            channels: channels.into_iter().collect(),
        }
    }

    pub fn from_slice(data: &'a mut [f32]) -> Self {
        Self::new(std::iter::once(data))
    }

    pub fn channel(&mut self, index: usize) -> Option<&mut [f32]> {
        self.channels.get_mut(index).map(|c| &mut **c)
    }

    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
    }

    pub fn clear(&mut self) {
        for channel in &mut self.channels {
            for sample in channel.iter_mut() {
                *sample = 0.0;
            }
        }
    }

    pub fn fill(&mut self, value: f32) {
        for channel in &mut self.channels {
            for sample in channel.iter_mut() {
                *sample = value;
            }
        }
    }
}

/// Buffer pool for audio processing
pub struct BufferPool {
    buffers: Vec<Vec<f32>>,
    block_size: usize,
}

impl BufferPool {
    pub fn new(num_buffers: usize, block_size: usize) -> Self {
        let mut buffers = Vec::with_capacity(num_buffers);
        for _ in 0..num_buffers {
            buffers.push(vec![0.0; block_size]);
        }

        Self {
            buffers,
            block_size,
        }
    }

    pub fn get(&self, index: usize) -> Option<&[f32]> {
        self.buffers.get(index).map(|b| b.as_slice())
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut [f32]> {
        self.buffers.get_mut(index).map(|b| b.as_mut_slice())
    }

    pub fn clear(&mut self, index: usize) {
        if let Some(buffer) = self.buffers.get_mut(index) {
            buffer.fill(0.0);
        }
    }

    pub fn clear_all(&mut self) {
        for buffer in &mut self.buffers {
            buffer.fill(0.0);
        }
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }

    pub fn resize(&mut self, num_buffers: usize) {
        if num_buffers > self.buffers.len() {
            let to_add = num_buffers - self.buffers.len();
            for _ in 0..to_add {
                self.buffers.push(vec![0.0; self.block_size]);
            }
        } else if num_buffers < self.buffers.len() {
            self.buffers.truncate(num_buffers);
        }
    }
}
