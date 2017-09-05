export RUST_LOG=rust_eureka=debug
export RUST_BACKTRACE=full
export EUREKA_URI=http://localhost:8080/eureka
alias eureka_start='docker run -d -p 8080:8080 --name eureka netflixoss/eureka:1.1.147'
alias eureka_stop='docker stop eureka'
alias eureka_restart='docker restart eureka'
