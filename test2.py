
import neworder as no

class M(no.Model):
  def __init__(self):
    super().__init__(no.Timeline(0,1,[10]), no.MonteCarlo.deterministic_identical_stream)

m = no.Model(no.Timeline(0,1,[10]), no.MonteCarlo.deterministic_identical_stream)

no.run(m)