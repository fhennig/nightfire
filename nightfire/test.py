import numpy as np


class RunningAverager:
    def __init__(self, avg_span: int):
        self.avg_span = avg_span
        self.hist = [0] * avg_span
        self.var_hist = [0] * avg_span
        self.raw_hist = [0] * avg_span
        self.avg = 0
        self.var = 0

    def add_new_value(self, value):
        # update mean
        avg_summand = value * 1/self.avg_span
        self.avg += avg_summand
        self.hist.append(avg_summand)
        old_value = self.hist.pop(0)
        self.avg -= old_value
        # update var
        var_summand = ((value - self.avg) ** 2) / self.avg_span
        self.var += var_summand
        self.var_hist.append(var_summand)
        old_value = self.var_hist.pop(0)
        self.var -= old_value
        # control var
        self.raw_hist.pop(0)
        self.raw_hist.append(value)
        mean = sum(self.raw_hist) / len(self.raw_hist)
        control = sum([(v - mean)**2 for v in self.raw_hist]) / len(self.raw_hist)
        print(f"{self.var}\t{control}")
        
