// main.rs
use std::io;

use TemperatureConverter::{Temperature, TemperatureUnit, TEMP_OPTIONS};
use iced::{button, pick_list, text_input, Alignment, Button, Column, Container, Element,
Length, PickList, Sandbox, Settings, Text, TextInput, 
};

#[derive(Default)]
struct TemperatureConverterStruct {
    input_value: String,
    input_unit: TemperatureUnit,
    output_unit: TemperatureUnit,
    result: Option<f64>,
    input_unit_list: pick_list::State<TemperatureUnit>,
    output_unit_list: pick_list::State<TemperatureUnit>,
    convert_button: button::State,
    input_value_state: text_input::State,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    InputUnitChanged(TemperatureUnit),
    OutputUnitChanged(TemperatureUnit),
    ConvertPressed,
}

impl Sandbox for TemperatureConverterStruct {
    type Message = Message;

    fn new() -> Self {
        TemperatureConverterStruct::default()
    }

    fn title(&self) -> String {
        String::from("Temperature Converter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::InputUnitChanged(unit) => {
                self.input_unit = unit;
            }
            Message::OutputUnitChanged(unit) => {
                self.output_unit = unit;
            }
            Message::ConvertPressed => {
                if let Ok(value) = self.input_value.parse::<f64>() {
                    let temp = Temperature::new(value, self.input_unit);
                    let converted = temp.convert_to(self.output_unit);
                    self.result = Some(converted.value);
                } else {
                    self.result = None;
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let input = TextInput::new(
            &mut self.input_value_state,
            "Enter value of temperature...",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(20);

        let input_unit = PickList::new(
            &mut self.input_unit_list,
            &TemperatureUnit::ALL[..],
            Some(self.input_unit),
            Message::InputUnitChanged,
        );

        let output_unit = PickList::new(
            &mut self.output_unit_list,
            &TemperatureUnit::ALL[..],
            Some(self.output_unit),
            Message::OutputUnitChanged,
        );

        let convert_button = Button::new(&mut self.convert_button, Text::new("Convert"))
            .on_press(Message::ConvertPressed);

        let result_text = if let Some(result) = self.result {
            Text::new(format!("Result: {:.2}", result))
        } else {
            Text::new("Invalid input or calculation error")
        };

        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(input)
            .push(input_unit)
            .push(output_unit)
            .push(convert_button)
            .push(result_text)
            .into()
    }
}

fn main() {
    
    TemperatureConverterStruct::run(Settings::default()).expect("Exited unexpectedly");
}
