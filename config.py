
# test script for development, run with
# cargo run . examples/hello_world
# or, e.g.
# mpirun -n target/debug/neworder . examples/hello_world

import neworder as no

def func():
  no.log("func")
  no.log(3)
  no.log(1.2)
  return no.name() #+ 3 #no.foo(3) #sum_as_string(1,2)


# The pure python equivalent to the above is:
#   import greet
#   import neworder
#   neworder.greeter = greet.Greet()
no.initialisations = {
  "greeter": { "module": "greet", "class_": "Greet", "args": () }
}

