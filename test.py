
import neworder as no

#print(dir(no))

no.log("hello")

t = no.Timeline.null()
t = no.Timeline(2010,2020, [1,3])
no.log(t)
no.log(str(t))
no.log(t.__str__())

while not t.at_end():
  no.log((t.index(), t.at_checkpoint(), t.at_end()))
  t.next()
no.log((t.index(), t.at_checkpoint(), t.at_end()))
# for _ in t:
#   no.log()

m = no.Model(t)
no.log(m)