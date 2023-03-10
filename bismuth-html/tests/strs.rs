mod utils;
use crate::utils::*;

snapshot_str!(
    test_template_str,
    "Does default work?",
r#"
<!DOCTYPE html>
<html lang="">

<head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <meta name="head:count" content="4">
    <title>test</title>
</head>

<body>
    <p>Does default work?</p>
<br>

</body>

</html>
"#
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
