
# test script for development, run with
# cargo run . examples/hello_world
# or, e.g.
# mpirun -n target/debug/neworder . examples/hello_world

import neworder as no
import numpy as np

def func():
  no.log("func")
  no.log(3)
  no.log(1.2)

  no.log("rank=%d" % no.rank)
  no.log("size=%d" % no.size)
  no.log("indep=%s" % no.indep)
  no.log("seed=%d" % no.seed)

  no.log("distant past = %f" % no.distant_past())
  no.log("far future = %f" % no.far_future())
  no.log("never = %f" % no.never())
  no.log("1.0 is never? %s" % no.isnever(1.0))

  no.log("numpy is available? %s" % np.ones(10, dtype=bool)[0])

  no.timeline = no.Timeline(2020.0, 2030.0, [10])
  no.log("timeline=%f %d %f" % (no.timeline.time(), no.timeline.index(), no.timeline.dt()))

  # t = no.TestClass(0, 2020.0)
  # t.next()
  # no.log("TestClass x=%f" % t.foo())
  # #no.log("TestClass i=%d" % t.bar())

  return no.name() #+ 3 #no.foo(3) #sum_as_string(1,2)


# The pure python equivalent to the above is:
#   import greet
#   import neworder
#   neworder.greeter = greet.Greet()
no.initialisations = {
  "greeter": { "module": "greet", "class_": "Greet", "args": ("x", "y", "z") }
}

