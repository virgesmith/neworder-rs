#import numpy as np
import neworder as no

print(no.__version__)

no.verbose()

class Test(no.Model):
  def __init__(_self, timeline):
    super().__init__(timeline, no.MonteCarlo.deterministic_identical_stream)

m = no.Model(no.NoTimeline(), no.MonteCarlo.deterministic_identical_stream)

t = no.NoTimeline()
print(t)
print(m.timeline) # base class
print(m.mc)
print(m.mc.ustream(10))
#m = Test()
no.run(m)