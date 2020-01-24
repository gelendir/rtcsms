RtcSms
======

Project that will send an SMS with the time left before the next bus passes at its stop. Can only be used
with the Web APIs for the [Réseau de Transport de la Capitale](https://www.rtcquebec.ca/). Uses the web APIs
from voip.ms to manage receving and sending SMSes. 

I wrote this project as a means to learning how to code in rust. To make things more interesting I
decided to only use what was available in rust out-of-the-box, which meant writing from scratch my own
HTTP client/server and JSON parser. The only 2 exceptions to this rule were:

- TLS, because I don't know enough about cryptography to write my own encryption layer
- Date handling. I could have, but I was getting lazy and wanted to finish the project quickly

I didn't implement the complete HTTP and JSON specs, but wrote enough to get it working with the RTC and
voip.ms web APIs.
