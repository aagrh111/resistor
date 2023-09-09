use cursive::Cursive;
use cursive::views::Dialog;
use cursive::views::DummyView;
use cursive::views::TextView;
use cursive::views::Panel;
use cursive::views::RadioGroup;
use cursive::views::LinearLayout;
use cursive::views::Button;
use cursive::traits::Nameable;
use cursive::event::Key;
use cursive::menu;

use crate::colors::Color;
use crate::colors::Multiplier;

fn about_window(scr: &mut Cursive) {
	let layout = LinearLayout::vertical()
		.child(Panel::new(TextView::new(
			"Resistor v0.1. Программа для вычисления сопротивления\n\
				      резисторов по их цветовой маркировке."
		)))
		.child(Panel::new(
			TextView::new(
				"Copyright (C) 2023 {Tsar}\n<michail383krasnov@mail.ru>"
			)
		))
		.child(Panel::new(
			TextView::new("Сбербанк: 2202 2062 5233 5406")
			).title("Где мои деньги, Лебовски?"));

	let win = Dialog::around(layout)
		.title("О программе")
		.button("ОК", |s| { s.pop_layer(); });
	scr.add_layer(win);
}

fn donut_window(scr: &mut Cursive) {
	let layout = LinearLayout::vertical()
		.child(TextView::new(
			"Над программой работает голодный человек в одиночку. Если\n\
			          вы хотите отблагодарить его за проделанную работу, то,\n\
			          пожалуйста, отправьте ему на карту донат. Он будет очень рад\n\
			          и обязательно продолжит работу над Resistor :)"
		))
		.child(Panel::new(
			TextView::new("2202 2062 5233 5406"))
		.title("Сбербанк"))
		.child(TextView::new(
			"Resistor — свободное ПО, за которое не платят его\n\
			          разработчику. Обратная связь от пользователей и добровольные\n\
			          пожертвования помогают сохранить мотивацию работать над\n\
			          программой дальше. Также деньги пойдут на покупку нового\n\
			          оборудования (мне нужен новый ноутбук) и на обслуживание уже\n\
			          имеющегося."
		));

	let win = Dialog::around(layout)
		.title("ДОНАТ")
		.button("ОК", |s| { s.pop_layer(); });
	scr.add_layer(win);
}

pub fn main_window(scr: &mut Cursive) {
	scr.pop_layer();

	scr.menubar()
		// .add_subtree("Маркировка", menu::Tree::new())
		.add_subtree("Справка", menu::Tree::new()
			.leaf("ДОНАТ", donut_window)
			.leaf("О программе", about_window)
		);
	scr.add_global_callback(Key::F1, |s| s.select_menubar());
	scr.add_global_callback(Key::F10, donut_window);
	scr.add_global_callback(Key::Esc, |s| s.quit());
	scr.set_autohide_menu(false);

	let mut colors1 = RadioGroup::new();
	let mut colors2 = RadioGroup::new();
	let mut colors3 = RadioGroup::new();
	let mut colors4 = RadioGroup::new();

	let work_layout = LinearLayout::horizontal()
		.child(Panel::new(
			LinearLayout::vertical()
				.child(colors1.button(Color::Black, Color::Black.to_str()))
				.child(colors1.button(Color::Brown, Color::Brown.to_str()))
				.child(colors1.button(Color::Red, Color::Red.to_str()))
				.child(colors1.button(Color::Orange, Color::Orange.to_str()))
				.child(colors1.button(Color::Yellow, Color::Yellow.to_str()))
				.child(colors1.button(Color::Green, Color::Green.to_str()))
				.child(colors1.button(Color::Blue, Color::Blue.to_str()))
				.child(colors1.button(Color::Purple, Color::Purple.to_str()))
				.child(colors1.button(Color::Gray, Color::Gray.to_str()))
				.child(colors1.button(Color::White, Color::White.to_str()))
		).title("Полоса №1"))
		.child(Panel::new(
			LinearLayout::vertical()
				.child(colors2.button(Color::Black, Color::Black.to_str()))
				.child(colors2.button(Color::Brown, Color::Brown.to_str()))
				.child(colors2.button(Color::Red, Color::Red.to_str()))
				.child(colors2.button(Color::Orange, Color::Orange.to_str()))
				.child(colors2.button(Color::Yellow, Color::Yellow.to_str()))
				.child(colors2.button(Color::Green, Color::Green.to_str()))
				.child(colors2.button(Color::Blue, Color::Blue.to_str()))
				.child(colors2.button(Color::Purple, Color::Purple.to_str()))
				.child(colors2.button(Color::Gray, Color::Gray.to_str()))
				.child(colors2.button(Color::White, Color::White.to_str()))
		).title("Полоса №2"))
		.child(Panel::new(
			LinearLayout::vertical()
				.child(colors3.button(Color::Black, Color::Black.to_str()))
				.child(colors3.button(Color::Brown, Color::Brown.to_str()))
				.child(colors3.button(Color::Red, Color::Red.to_str()))
				.child(colors3.button(Color::Orange, Color::Orange.to_str()))
				.child(colors3.button(Color::Yellow, Color::Yellow.to_str()))
				.child(colors3.button(Color::Green, Color::Green.to_str()))
				.child(colors3.button(Color::Blue, Color::Blue.to_str()))
				.child(colors3.button(Color::Purple, Color::Purple.to_str()))
				.child(colors3.button(Color::Gray, Color::Gray.to_str()))
				.child(colors3.button(Color::White, Color::White.to_str()))
		).title("Полоса №3"))
		.child(Panel::new(
			LinearLayout::vertical()
				.child(colors4.button(Multiplier::Black, Multiplier::Black.to_str()))
				.child(colors4.button(Multiplier::Brown, Multiplier::Brown.to_str()))
				.child(colors4.button(Multiplier::Red, Multiplier::Red.to_str()))
				.child(colors4.button(Multiplier::Orange, Multiplier::Orange.to_str()))
				.child(colors4.button(Multiplier::Yellow, Multiplier::Yellow.to_str()))
				.child(colors4.button(Multiplier::Green, Multiplier::Green.to_str()))
				.child(colors4.button(Multiplier::Blue, Multiplier::Blue.to_str()))
				.child(colors4.button(Multiplier::Gold, Multiplier::Gold.to_str()))
				.child(colors4.button(Multiplier::Silver, Multiplier::Silver.to_str()))
		).title("Полоса №4"));

	let status_bar = LinearLayout::horizontal()
		.child(Panel::new(LinearLayout::horizontal()
			.child(Button::new("Вычислить", move |s| {
				let color1 = colors1
					.selection()
					.to_usize();
				let color2 = colors2
					.selection()
					.to_usize();
				let color3 = colors3
					.selection()
					.to_usize();
				let color4 = colors4
					.selection()
					.to_multiplier();
				let color: f32 = format!("{color1}{color2}{color3}")
					.trim()
					.parse()
					.unwrap();

				let value = color * color4;
				s.call_on_name("result", |txt: &mut TextView| {
					txt.set_content(format!(
						"Сопротивление: {} {}", value,
						colors4.selection().to_label()
					));
				});
			}))
			.child(DummyView)
			.child(Button::new("Выйти", |s| s.quit()))
		))
		.child(Panel::new(
			TextView::new("здесь появятся результаты вычислений")
				.with_name("result")
		));

	let win = Dialog::around(LinearLayout::vertical()
		.child(work_layout)
		.child(status_bar)
	).title("Resistor");

	scr.add_layer(win);
}