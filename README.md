[//]: # (auto_md_to_doc_comments segment start A)

# authentication_database_web_ui

[//]: # (auto_cargo_toml_to_md start)

**09. Tutorial to code authentication for database_web_ui (2022-10)**  
***version: 0.1.01 date: 2022-10-10 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/authentication_database_web_ui)***  

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)

[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-230-green.svg)](https://github.com/bestia-dev/authentication_database_web_ui/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-9-blue.svg)](https://github.com/bestia-dev/authentication_database_web_ui/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-28-purple.svg)](https://github.com/bestia-dev/authentication_database_web_ui/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/authentication_database_web_ui/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/authentication_database_web_ui/)

[//]: # (auto_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/authentication_database_web_ui/blob/main/LICENSE) [![Rust](https://github.com/bestia-dev/authentication_database_web_ui/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/authentication_database_web_ui/) ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/0.svg)

Hashtags: #rust #rustlang #tutorial  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Intro

In [previous tutorials](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering), we created a working prototype for a web application that can create, read, update and delete (CRUD) data in a Postgres database.  
We will continue to develop this project in the 9th part of the [Rust tutorial series](https://www.youtube.com/channel/UCitt3zFHK2jDetDh6ezI05A). Today, we will add some code for user authentication.  

This project has also a youtube video tutorial. Watch it:
<!-- markdownlint-disable MD033 -->
[<img src="https://bestia.dev/youtube/authentication_database_web_ui.png" width="400px">](https://bestia.dev/youtube/authentication_database_web_ui.html)
<!-- markdownlint-enable MD033 -->

## Motivation

If a web application manipulates data in a database it needs some kind of authentication. You can not leave it completely open to the internet. All kinds of nasty things can occur. It is a World Wild West out there!
Authentication is the process of recognizing the user's identity. This is later used to authorize the user to what can he/she do with the database.  

## Webassembly/WASM for client code execution

We will introduce Webassembly/WASM to run code on the client inside the browser. The same could be done with Javascript, but I despise it deeply and will use Rust compiled to WASM instead.  
The client WASM will talk to the server with json messages inside POST requests bodies and response bodies.

## Workspace

Rust has the concept of "workspace" to group more projects together. But it does not work well if there is a mix of server projects and WASM projects.  
This is not a standard cargo workspace. It does not have Cargo.toml in the workspace folder.
Because of one WASM project I cannot use the profile `panic="abort"` on all the members.
There is no workaround as of 2022-10-20.  
We can use cargo-auto to automate the tasks for every member individually.
Then we code the automation_tasks_rs on the workspace level to call all members tasks together.
The presence of Cargo-auto.toml or Cargo.toml is used by cargo-auto to recognize project folders.
No config exist for now for Cargo-auto.toml. Maybe one day we will need to add something.

## Authentication and session cookie

The topic of authentication can get very complicated very quickly. For this tutorial, we will use a very "simple" method with sessions and cookies. It sounds old-school and it is, but it is simple, effective and it has been strengthened lately. All communication must be secured by SSL/TLS because otherwise a hacker can intercept and read secrets. The cookies MUST be SameSite to avoid nasty hacker attacks. This session cookie is a "necessary cookie" and is exempt from the European Privacy GDPR Laws.

A new user will sign up on a "user_sign_up page" with 3 input strings: user_email and 2 times the same password.  
The client (WASM) checks that the fields are not empty, if the user_email looks like an email, if the 2 passwords are identical and if they are complicated enough.  
The password is salted and hashed as described in the mechanism [SCRAM](https://en.wikipedia.org/wiki/Salted_Challenge_Response_Authentication_Mechanism).  
The user_email and hashed password are sent to the server. Only the user knows the password. The administrator or hacker will see only the salted hash.  
The data is temporarily stored in table "user_sign_up".  
The server sends an email to the user with a link for verification. After verification, the record is updated as verified and an email is sent to the administrator for the new sign_up. The administrator can choose to manually grant access to the new user or not. This step can later be automated if needed. Now the user data is moved to the table authn_login. The app sends an email to the user to inform him/her that access is granted.  
The user opens the "authn_login page" with 2 input strings: user_email and password. The client checks that the fields are not empty and if the user_email looks like an email. The client sends the first msg to the server with the user_email. The server returns the salt for hashing. The client calculates the salted hash and sends it. The server checks if the hash is the same as in the database. If not, a red alert is shown, but the user_email and password are not deleted from the authn_login_page. The failed login updates the field failed_logins in authn_login. If failed logins are more than 3 then the speed of returning alerts is drastically slowed down to slow down dictionary or brute force attacks. After 20 failed logins, the user is blocked and an email is sent to him and the administrator. The administrator will contact the user and eventually manually unblock the user.  
If successful the server inserts a new random session_id into the table user_session and sends a cookie to the client (and updates failed_login to 0). The session is ephemeral. It expires after 5 minutes of the last request.  
This session cookie will be attached to every request sent from this client. The server will check in the table user_session that the session_id is alive and grant access. The session_id has an expiration date_time and this is updated on every request. If the session_id expires it is deleted from the table and subsequent requests will fail. The user will need to log in again. Session_id does not need to be saved in a persistent table for now. I will use some kind of cache in memory for performance. Alternatively, we could use [Redis distributed cache](https://redis.io/), which is faster than database access.

## Sending email

It is not easy to send emails over SMTP anymore because of spam. There is so much spam, that big email providers invented the "email deliverability reputation" system, which makes it difficult for smaller senders to not be flagged as spam. So the solution is to use some free email providers like MailGun, MailJet, Mailersend or Sendgrid. The email communication between a web app and its user is called "transactional email" and is specific because it needs to be fast and reliable. A transactional email is a type of email message that’s triggered by a specific action on a website or mobile app. Some common examples of transactional emails include password resets, order confirmations, automated abandoned cart emails, account notifications, social media updates, welcome emails, and any other confirmation emails that are sent via automation. These automated emails are typically sent programmatically through an email API or SMTP server.



Save your html files in UTF-8 encoding without the byte-order mark (BOM).

The declaration is not an HTML tag. It is an "information" to the browser about what document type to expect.

TODO: random salt, random session_id

## Debugger

In VSCode install the extension CodeLLDB.  
The container must be run with some config that allows debugging.
--cap-add=SYS_PTRACE --security-opt seccomp=unconfined


## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read crev reviews quickly on the web:  
<https://web.crev.dev/rust-reviews/crates/>  

## open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating on my [Paypal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)  

[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//]: # (auto_md_to_doc_comments segment end A)
