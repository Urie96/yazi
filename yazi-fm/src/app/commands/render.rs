use std::sync::atomic::Ordering;

use anyhow::Result;
use ratatui::backend::Backend;

use crate::{app::App, lives::Lives, root::{Root, COLLISION}};

impl App {
	pub(crate) fn render(&mut self) -> Result<()> {
		let Some(term) = &mut self.term else {
			return Ok(());
		};

		let collision = COLLISION.swap(false, Ordering::Relaxed);
		let frame = term.draw(|f| {
			Lives::scope(&self.cx, |_| {
				f.render_widget(Root::new(&self.cx), f.size());
			});

			if let Some((x, y)) = self.cx.cursor() {
				f.set_cursor(x, y);
			}
		})?;
		if !COLLISION.load(Ordering::Relaxed) {
			if collision {
				// Reload preview if collision is resolved
				self.cx.manager.peek(true);
			}
			return Ok(());
		}

		let mut patch = vec![];
		for x in frame.area.left()..frame.area.right() {
			for y in frame.area.top()..frame.area.bottom() {
				let cell = frame.buffer.get(x, y);
				if cell.skip {
					patch.push((x, y, cell.clone()));
				}
			}
		}

		term.backend_mut().draw(patch.iter().map(|(x, y, cell)| (*x, *y, cell)))?;
		if let Some((x, y)) = self.cx.cursor() {
			term.show_cursor()?;
			term.set_cursor(x, y)?;
		}
		term.backend_mut().flush()?;
		Ok(())
	}
}
