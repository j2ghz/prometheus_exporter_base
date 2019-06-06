pub trait RenderToPrometheus {
    fn render(&self) -> String;
}

pub struct PrometheusCounter<'a> {
    pub counter_name: &'a str,
    pub counter_type: &'a str,
    pub counter_help: &'a str,
    pub attributes: Vec<(&'a str, String)>,
}

impl<'a> PrometheusCounter<'a> {
    pub fn new(
        counter_name: &'a str,
        counter_type: &'a str,
        counter_help: &'a str,
    ) -> PrometheusCounter<'a> {
        PrometheusCounter {
            counter_name,
            counter_type,
            counter_help,
            attributes: Vec::new(),
        }
    }

    pub fn render_header(&self) -> String {
        format!(
            "# HELP {} {}\n# TYPE {} {}\n",
            self.counter_name, self.counter_help, self.counter_name, self.counter_type
        )
    }

    pub fn render_counter<N>(&self, value: N) -> String
    where
        N: std::fmt::Display,
    {
        if self.attributes.is_empty() {
            format!("{} {}\n", self.counter_name, value.to_string())
        } else {
            let mut s = format!("{}{{", self.counter_name);

            let mut first = true;
            for (key, val) in self.attributes.iter() {
                if !first {
                    s.push_str(",");
                } else {
                    first = false;
                }

                s.push_str(&format!("{}=\"{}\"", key, val));
            }

            s.push_str(&format!("}} {}\n", value.to_string()));
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let pc = PrometheusCounter::new("pippo_total", "counter", "Number of pippos");

        assert_eq!(
            pc.render_header(),
            "# HELP pippo_total Number of pippos\n# TYPE pippo_total counter\n"
        );
    }

    #[test]
    fn test_attributes() {
        let mut pc = PrometheusCounter::new("pippo_total", "counter", "Number of pippos");
        let mut number = 0;

        pc.attributes.push(("food", "chicken".to_owned()));
        pc.attributes.push(("instance", "".to_owned()));

        for _ in 0..4 {
            pc.attributes[1].1 = number.to_string();
            let ret = pc.render_counter(&*number.to_string());

            assert_eq!(
                ret,
                format!(
                    "pippo_total{{food=\"chicken\",instance=\"{}\"}} {}\n",
                    number, number
                )
            );
            number += 1;
        }
    }

    #[test]
    fn test_no_attributes() {
        let pc = PrometheusCounter::new("gigino_total", "counter", "Number of giginos");
        assert_eq!(pc.render_counter(100), format!("gigino_total {}\n", 100));
    }
}
