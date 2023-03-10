mod utils;
use crate::utils::*;

snapshot_str!(
    test_template_str,
    "Does default work?",
    "
<p>Does default work?</p>
<br>
"
);

snapshot_str!(
    test_template_str_1,
    "Does default work?",
    "
---
kind: test
values:
    - value_1: test value 1
    - value_2: test value 2
---
",
"
Test template: test value 1 test value 2
<br>
<p>Does default work?</p>
<br>
"
);
