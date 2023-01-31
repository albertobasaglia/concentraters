# ConcentrateRS

Simple tool for adding stuff to /etc/hosts file to prevent you from accessing it.

## Todo

- Add the ability to backup the original /etc/hosts file.
- Read (and create if it doesn't exist) a file containing a config for the program.
  - Sites to block
- Decide one of the following
  - Keep the program running with a cli
  - Make it operate with a temp file (containing the status which initally
  could just be something like 'on'/'off') and use it with parameters (I like
  this way more).
    - concentraters on
    - concentraters off
    - concentraters block youtube.com
    - concentraters unblock youtube.com
