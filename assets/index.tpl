{{> header }}

<h2><code>pseudo-lang</code></h2>

<p>
A flexible language that anyone can write.
</p>

<p>
Pseudo compiles in the cloud to C++, python, javascript, rust, and many other targets!
You'll be off in no time: just write some pseudocode and run the compiler with your specified language.
</p>

<h4>Downloads</h4>
<div id='downloads'>
        <ul>
                <li><a href='#'><code>pseudoc</code> for Ubuntu 14.04</a></li>
                <li><a href='#'><code>pseudoc</code> for Mac OSX</a></li>
                <li><i>Windows someday..</i></li>
        </ul>
</div>

<h4>Examples</h4>
<p>sending an SMS:</p>
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

<p>downloading a file:</p>
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

<p>drawing a picture:</p>
<pre>
</pre>

<h4>Develop</h4>
<p>
The <code>pseudoc</code> compiler can be built with <a href="https://www.rust-lang.org">rust</a>
following the instructions <a href="http://github.com/yosemitebandit/pseudo#readme">on github</a>.
</p>

{{> link-bar }}
{{> footer }}
