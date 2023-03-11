mod utils;
use crate::utils::*;

snapshot_path!(test_path, "./testdata/test/test.md", r#"<!DOCTYPE html>
<html lang="">

<head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <meta name="head:count" content="4">
    <title>This is a test</title>
</head>

<body>

<br>

<br>
<h1> h1 Heading 8-</h1>
<br>
<h2> h2 Heading</h2>
<br>
<h3> h3 Heading</h3>
<br>
<h4> h4 Heading</h4>
<br>
<h5> h5 Heading</h5>
<br>
<h6> h6 Heading</h6>
<br>

<br>

<br>
<h2> Horizontal Rules</h2>
<br>

<br>
<hr>
<br>

<br>
<p>!!!!!! ???? ,,  -- ---</p>
<br>

<br>
<p>"Smartypants, double quotes" and 'single quotes'</p>
<br>

<br>

<br>
<h2> Emphasis</h2>
<br>

<br>
<b>This is bold text</b>
<br>

<br>
<b>This is bold text</b>
<br>

<br>
<i>This is italic text</i>
<br>

<br>
<i>This is italic text</i>
<br>

<br>
<h2> Blockquotes</h2>
<br>

<br>

<br>
<blockquote> Blockquote...</blockquote>
<br>

<br>

<br>
<h2> Lists</h2>
<br>

<br>
<p>Unordered</p>
<br>

<br>
<li class="item">	 Sub-lists are made by indenting 2 spaces:</li>
<br>
<li class="item">		 Marker character change forces new list start:</li>
<br>
<li class="item">			 Facilisis in pretium nisl aliquet</li>
<br>
<li class="item">			 Nulla volutpat aliquam velit</li>
<br>
<li class="item">	 Very easy!</li>
<br>

<br>
<p>Ordered</p>
<br>

<br>
<li class="num-list">	1. Lorem ipsum dolor sit amet</li>
<br>
<li class="num-list">	2. Consectetur adipiscing elit</li>
<br>
<li class="num-list">	3. Integer molestie lorem at massa</li>
<br>

<br>

<br>
<li class="num-list">	1. You can use sequential numbers...</li>
<br>
<li class="num-list">	1. ...or keep all the numbers as <div class="inline-code">1.</div></li>
<br>

<br>
<p>Start numbering with offset:</p>
<br>

<br>
<li class="num-list">	57. foo</li>
<br>
<li class="num-list">	1. bar</li>
<br>

<br>

<br>
<h2> Code</h2>
<br>

<br>
<p>Inline <div class="inline-code">code</div></p>
<br>

<br>
<p>Block code "fences"</p>
<br>

<br>
<pre style="background-color:#ffffff;">
<span style="color:#323232;">
</span><span style="color:#323232;">Sample text here...
</span></pre>

<br>

<br>
<p>Syntax highlighting</p>
<br>

<br>
<pre style="background-color:#ffffff;">
<span style="color:#323232;">
</span><span style="font-weight:bold;color:#a71d5d;">var </span><span style="font-weight:bold;color:#795da3;">foo </span><span style="font-weight:bold;color:#a71d5d;">= function </span><span style="color:#323232;">(bar) {
</span><span style="color:#323232;">  </span><span style="font-weight:bold;color:#a71d5d;">return </span><span style="color:#323232;">bar</span><span style="font-weight:bold;color:#a71d5d;">++</span><span style="color:#323232;">;
</span><span style="color:#323232;">};
</span><span style="color:#323232;">
</span><span style="color:#795da3;">console</span><span style="color:#323232;">.</span><span style="color:#0086b3;">log</span><span style="color:#323232;">(foo(</span><span style="color:#0086b3;">5</span><span style="color:#323232;">));
</span></pre>

<br>

<br>
<h2> Links</h2>
<br>

<br>
<a target="/test" >
<br>

<br>
<a target="example.com" target="blank">
<br>

<br>
<h2> Images</h2>
<br>

<br>
<img src="/test.png" alt="Minion">
<br>
<img src="example.com" alt="Stormtroopocat">
<br>

<br>
<p>Like links, Images also have a footnote style syntax</p>
<br>

<br>
<h2> Custom stuff:</h2>
<br>
<blockquote> TODO:</blockquote>
<br>
key: value
<br>

<br>
<h3> Tex:</h3>
<br>

<br>
<p>inline <span class="katex"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><mi>e</mi><mo>=</mo><mi>m</mi><msup><mi>c</mi><mn>2</mn></msup></mrow><annotation encoding="application/x-tex">e = mc^2</annotation></semantics></math></span></p>
<br>

<br>
<p>$</p>
<br>
<p>e = mc^2</p>
<br>
<p>$</p>
<br>

<br>

</body>

</html>
"#);
