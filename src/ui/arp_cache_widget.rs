use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Widget},
};

use crate::arp_cache::ArpEntry;

#[derive(Default)]
pub struct ArpCacheWidget<'a> {
    style: Style,
    block: Option<Block<'a>>,
    entries: Vec<ArpEntry>,
}

impl<'a> ArpCacheWidget<'a> {
    pub fn entries(mut self, vec: Vec<ArpEntry>) -> Self {
        self.entries = vec;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Widget for ArpCacheWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let lines: Vec<String> = self
            .entries
            .iter()
            .map(|entry| format!(" {} at {} ", entry.ip(), entry.mac()))
            .collect();

        let max_width = text_area.width as usize;
        let mut wrapped_lines: Vec<String> = vec![];
        for mut line in lines.into_iter() {
            while line.len() > max_width {
                let overflow = line.split_off(max_width);
                wrapped_lines.push(line);
                line = overflow;
            }
            wrapped_lines.push(line);
        }

        for (i, line) in wrapped_lines.iter().enumerate() {
            let len = line.len();
            buf.set_stringn(
                text_area.left(),
                text_area.top() + i as u16,
                line,
                len,
                self.style,
            );
        }
    }
}
