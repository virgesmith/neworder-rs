
import neworder as no

print(dir(no))

no.log("rank=%d size=%d" % (no.mpi.rank(), no.mpi.size()))

t = no.Timeline.null()
t = no.Timeline(2010,2020, [1,3])
no.log(t)
no.log(str(t))
no.log(t.__str__())

no.log(no.time.isnever(1.234))
no.log(no.time.isnever(no.time.distant_past()))
no.log(no.time.isnever(no.time.far_future()))
no.log(no.time.isnever(no.time.never()))

while not t.at_end():
  no.log((t.index(), t.at_checkpoint(), t.at_end()))
  t.next()
no.log((t.index(), t.at_checkpoint(), t.at_end()))
# for _ in t:
#   no.log()

m = no.Model(t, no.MonteCarlo.deterministic_identical_seed)

m.modify(no.mpi.rank())

try:
  m.step()
except NotImplementedError as e:
  no.log(e)
else:
  assert False, "expected error, didnt get one"

assert m.check()

try:
  m.checkpoint()
except NotImplementedError as e:
  no.log(e)
else:
  assert False, "expected error, didnt get one"

# not working. how to get (mut) ref to class member?
#no.log(m.timeline())
#no.log(m.mc())

class Test(no.Model):
  def __init__(self):
    super().__init__(no.Timeline.null(), no.MonteCarlo.deterministic_independent_seed)

# not working TypeError: Model.__new__() missing required positional argument: timeline
#t = Test()
