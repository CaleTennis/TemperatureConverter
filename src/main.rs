// main.rs

const WINDOW_NAME: &str = "Temperature Converter";
const INPUT_PLACEHOLDER_TEXT: &str = "Input temperature here...";

use TemperatureConverter::{format_with_commas, Temperature, TemperatureUnit};
use iced::{
    pick_list, text_input, Alignment, Column, Element, 
    PickList, Row, Sandbox, Settings, Text, TextInput 
};

#[derive(Default)]
struct TemperatureConverterStruct {
    input_value: String,
    input_unit: TemperatureUnit,
    output_unit: TemperatureUnit,
    result: Option<f64>,
    input_unit_list: pick_list::State<TemperatureUnit>,
    output_unit_list: pick_list::State<TemperatureUnit>,
    input_value_state: text_input::State,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    InputUnitChanged(TemperatureUnit),
    OutputUnitChanged(TemperatureUnit),
    Convert
}

impl Sandbox for TemperatureConverterStruct {
    type Message = Message;

    fn new() -> Self {
        TemperatureConverterStruct::default()
    }

    fn title(&self) -> String {
        String::from(WINDOW_NAME)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
                self.update(Message::Convert);
            }
            Message::InputUnitChanged(unit) => {
                self.input_unit = unit;
                self.update(Message::Convert);
            }
            Message::OutputUnitChanged(unit) => {
                self.output_unit = unit;
                self.update(Message::Convert);
            }
            Message::Convert => {
                if let Ok(value) = self
                                            .input_value
                                            .replace(',',"")
                                            .replace(' ', "")
                                            .parse::<f64>() {
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
            INPUT_PLACEHOLDER_TEXT,
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

        let result_text = if let Some(result) = self.result {
            Text::new(format_with_commas(result, self.output_unit))
        } else {
            Text::new("")
        };

        // Bread and Butter of Display

        Column::new()
            .padding(80)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(input)
            .push(
                Row::new()
                    .padding(20)
                    .align_items(Alignment::Center)
                    .spacing(20)
                    .push(input_unit)
                    .push(output_unit)
                )
            .push(result_text)
            .into()
    }

}

fn main() {

    TemperatureConverterStruct::run(Settings::default()).expect("Exited unexpectedly");
}
