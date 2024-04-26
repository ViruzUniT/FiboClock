#[cfg(test)]
mod tests {
    use crate::get_time;
    use crate::Color;
    use crate::Field;
    use crate::Time;

    #[test]
    fn first_blue() {
        let mut fields: [Field; 5] = [
            Field::new(1),
            Field::new(1),
            Field::new(2),
            Field::new(3),
            Field::new(5),
        ];
        fields[0].color = Color::Blue;
        let time = get_time(&fields).unwrap();
        let correct_time = Time { hour: 1, minute: 5 };

        assert!(
            time.hour == correct_time.hour && time.minute == correct_time.minute,
            "time should look like {:?} but it looks like {:?}",
            correct_time,
            time
        )
    }

    #[test]
    fn six_forty() {
        let mut fields: [Field; 5] = [
            Field::new(1),
            Field::new(1),
            Field::new(2),
            Field::new(3),
            Field::new(5),
        ];
        fields[0].color = Color::Red;
        fields[3].color = Color::Green;
        fields[4].color = Color::Blue;
        let correct_time = Time {
            hour: 6,
            minute: 40,
        };
        let time = get_time(&fields).unwrap();

        assert!(
            time.hour == correct_time.hour && time.minute == correct_time.minute,
            "time should look like {:?} but it looks like {:?}",
            correct_time,
            time
        )
    }
    #[test]
    fn twelfe_forty() {
        let mut fields: [Field; 5] = [
            Field::new(1),
            Field::new(1),
            Field::new(2),
            Field::new(3),
            Field::new(5),
        ];
        fields[0].color = Color::Red;
        fields[1].color = Color::Red;
        fields[2].color = Color::Red;
        fields[3].color = Color::Blue;
        fields[4].color = Color::Blue;
        let correct_time = Time {
            hour: 12,
            minute: 40,
        };
        let time = get_time(&fields).unwrap();

        assert!(
            time.hour == correct_time.hour && time.minute == correct_time.minute,
            "time should look like {:?} but it looks like {:?}",
            correct_time,
            time
        )
    }
    #[test]
    fn ten_thirtyfive() {
        let mut fields: [Field; 5] = [
            Field::new(1),
            Field::new(1),
            Field::new(2),
            Field::new(3),
            Field::new(5),
        ];
        fields[2].color = Color::Blue;
        fields[3].color = Color::Red;
        fields[4].color = Color::Blue;
        let correct_time = Time {
            hour: 10,
            minute: 35,
        };
        let time = get_time(&fields).unwrap();

        assert!(
            time.hour == correct_time.hour && time.minute == correct_time.minute,
            "time should look like {:?} but it looks like {:?}",
            correct_time,
            time
        )
    }
    #[test]
    fn tester() {
        let mut fields: [Field; 5] = [
            Field::new(1),
            Field::new(1),
            Field::new(2),
            Field::new(3),
            Field::new(5),
        ];
        fields[3].color = Color::Green;
        fields[4].color = Color::Red;
        let correct_time = Time {
            hour: 5,
            minute: 15,
        };
        let time = get_time(&fields).unwrap();

        assert!(
            time.hour == correct_time.hour && time.minute == correct_time.minute,
            "time should look like {:?} but it looks like {:?}",
            correct_time,
            time
        )
    }
}
