{{> header }}

<h2><code>pseudo-lang</code></h2>

<p>
A flexible language that anyone can write.
</p>

<p>
Pseudo compiles in the cloud to C++, python, javascript, rust, and many other targets!
You'll be off in no time: just write some pseudocode and run the compiler with your specified output language.
</p>


<h4>Downloads</h4>
<div id='downloads'>
        <ul>
                <li>
                    <a href='https://github.com/yosemitebandit/pseudo/releases/download/v0.2.0/pseudoc-trusty'><code>pseudoc</code> for Ubuntu 14.04</a> --
                    you may also need to <code>apt-get install libpq5</code>

                </li>
                <li><a href='https://github.com/yosemitebandit/pseudo/releases/download/v0.2.0/pseudoc-mac-osx'><code>pseudoc</code> for Mac OSX</a></li>
                <li><i>pseudoc for Windows..someday</i></li>
        </ul>
</div>


<h4>Examples</h4>
<p>sending an SMS with python:</p>
<pre>
$ cat sms.pseudo
ask the user for a number
send an SMS to that number saying "hello!"
leave some API details for me to fill in later..

$ pseudoc sms.pseudo --output=sms.py --language=python

$ cat sms.py
import sys
from twilio.rest import TwilioRestClient

account = "AC123456"
token = "zyxwvut"
from_number = "+1555123456"
client = TwilioRestClient(account, token)

to_number = sys.argv[1]
client.messages.create(to=to_number, from_=from_number, body="hello!")
</pre>


<p>downloading a file with rust:</p>
<pre>
$ cat rainfall.pseudo
download some rainfall data
preferably as a CSV from NOAA

$ pseudoc rainfall.pseudo --output=rainfall.rs --language=rust

$ cat rainfall.rs
extern crate hyper;

use std::io::prelude::*;
use hyper::Client;

const URL: &'static str = "http://www1.ncdc.noaa.gov/pub/data/cdo/samples/PRECIP_HLY_sample_csv.csv";

fn main() {
    let client = Client::new();
    let mut response = client.get(URL).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    println!("{}", body);
}
</pre>


<p>drawing a picture with javascript:</p>
<pre>
$ cat drawing.pseudo
draw a picture
of a green rectangle
and five purple circles in various places

$ pseudoc drawing.pseudo --output=drawing.js --language=javascript

$ cat drawing.js
function draw() {
  var canvas = document.getElementById('canvas');
  if (canvas.getContext) {
    var context = canvas.getContext('2d');
    context.fillStyle = "green";
    context.fillRect(100, 100, 400, 200);

    context.fillStyle = "purple";
    for (var i=0; i<5; i++) {
      var x = Math.random() * 800;
      var y = Math.random() * 800;
      var radius = Math.random() * 100;
      context.beginPath();
      context.arc(x, y, radius, 0, Math.PI*2, true);
      context.fill();
    }
  }
};
</pre>


<h4>Develop</h4>
<p>
The <code>pseudoc</code> compiler is built with <a href="https://www.rust-lang.org">rust</a> --
to build it yourself follow the instructions <a href="http://github.com/yosemitebandit/pseudo#readme">on github</a>.
</p>


{{> link-bar }}
{{> footer }}
