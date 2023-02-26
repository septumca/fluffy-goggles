trait TriggerAble {
    fn execute(&mut self, changes: &[TokenChange]) -> Vec<TokenChange>;
}

struct Trigger {}