
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

  no.log("rank=%d" % no.rank())
  no.log("size=%d" % no.size())
  no.log("indep=%s" % no.INDEP)
  no.log("seed=%d" % no.SEED)

  no.log("distant past = %f" % no.distant_past())
  no.log("far future = %f" % no.far_future())
  no.log("never = %f" % no.never())
  no.log("1.0 is never? %s" % no.isnever(1.0))

  no.log("numpy is available? %s" % np.ones(10, dtype=bool)[0])

  no.timeline = no.Timeline(2020.0, 2030.0, [5,10]) # NB timeline info NOT synced with rust!
  #no.timeline = no.Timeline.null()
  # no.log("timeline=%f %d %f" % (no.timeline.time(), no.timeline.index(), no.timeline.dt()))
  # no.log("timeline chk/end=%s %s" % (no.timeline.at_checkpoint(), no.timeline.at_end()))
  
  #mc = no.MonteCarlo(0,1,true)

  #no.log(no.mc.seed())
  # no.log(no.mc.ustream(5)) 
  # t = no.TestClass(0, 2020.0)
  # t.next()
  # no.log("TestClass x=%f" % t.foo())
  # #no.log("TestClass i=%d" % t.bar())

  #return no.name() #+ 3 #no.foo(3) #sum_as_string(1,2)


# The pure python equivalent to the above is:
#   import greet
#   import neworder
#   neworder.greeter = greet.Greet()
no.initialisations = {
  "greeter": { "module": "greet", "class_": "Greet", "args": ("x", "y", "z") }
}

no.modifiers = [ ] 

no.transitions = { } #TODO fix"who": "greeter.get_name()" }

no.checks = { } #"dummy": "True" }

no.checkpoints = { 
  "null": "pass",
  "mc": "neworder.log(neworder.mc.ustream(5))"
 }

no.timeline = no.Timeline(2020.0, 2030.0, [5,10])

assert True

#func()
