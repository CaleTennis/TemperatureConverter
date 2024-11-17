// main.rs

const WINDOW_NAME: &str = "Temperature Converter";
const INPUT_PLACEHOLDER_TEXT: &str = "Input temperature here...";
const MAX_INPUT_DIGITS: usize = 20;
const WINDOW_DIMENSIONS: (u32, u32) = (650,400);
const TITLE_TEXT: &str = "TEMPERATURE CONVERTER";

use TemperatureConverter::{format_with_commas, Temperature, TemperatureUnit,warn_below_absolute_zero};
use iced::{pick_list, slider, text_input, window, Alignment, Column, Element, Length, PickList, Row, Sandbox, Settings, Slider, Text, TextInput 
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
    decimal_precision_value: u8,
    decimal_precision_slider_state: slider::State,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    InputUnitChanged(TemperatureUnit),
    OutputUnitChanged(TemperatureUnit),
    PrecisionSliderChanged(u8),
    Convert,
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
                
                if value.len() <= MAX_INPUT_DIGITS {
                    self.input_value = value;
                }

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
            Message::PrecisionSliderChanged(value) => {
                self.decimal_precision_value = value;
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

        let title = Text::new(TITLE_TEXT).size(40);

        let input = TextInput::new(
            &mut self.input_value_state,
            INPUT_PLACEHOLDER_TEXT,
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(25)
        .width(Length::Units(450));

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

        let warning_text;
        let result_text = if let Some(result) = self.result {
            warning_text = Text::new(warn_below_absolute_zero(result, self.output_unit));
            Text::new(format!("{}",format_with_commas(result, self.output_unit, self.decimal_precision_value)))
        } else {
            warning_text = Text::new("");
            Text::new("")
        };
        
       let precision_slider = Slider::new (
            &mut self.decimal_precision_slider_state,
            0..=10,
            self.decimal_precision_value,
            Message::PrecisionSliderChanged,
       );
       
        // Bread and Butter of Display

        Column::new()
            .padding(80)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(title)
            .push(input)
            .push(
                Row::new()
                    .padding(20)
                    .align_items(Alignment::Center)
                    .spacing(20)
                    .push(input_unit)
                    .push(output_unit)
                    .push(Column::new()
                        .push(Text::new(format!("Precision: {}", self.decimal_precision_value.to_string())))
                        .push(precision_slider)
                        .spacing(10)
                    )
                )
            .push(result_text)
            .push(warning_text)
            .into()
    }

}

fn main() {
    let settings = Settings {
        window: window::Settings {
            size: WINDOW_DIMENSIONS,
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };

    TemperatureConverterStruct::run(settings).expect("Exited unexpectedly");
}