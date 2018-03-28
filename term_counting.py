import numpy as np

class term:
    def __init__(self, factor, ds, pis, sigmas, deltas):
        self.factor = factor
        self.ds = ds
        self.pis = pis
        self.sigmas = sigmas
        self.deltas = deltas
        self.terminal = False
    def initialise(a, b, c, n):
        l = a+b+c
        ds = np.array([2 for n in range(a)] + [4 for n in range(b)] + [6 for n in range(c)])
        pis = n
        sigmas = np.zeros(l)
        deltas = np.zeros([l, l])
        return term(1, ds, pis, sigmas, deltas)

    #If option is -1, taking from pi and adding to sigmas
    def new_term(self, d_index, option):
        if option == -1:
            pi = self.pis - 1
            sigmas = self.sigmas.copy()
            sigmas[d_index] += 1
            ds = self.ds.copy() 
            ds[d_index] -= 1
            t = term(self.factor*self.pis, ds, pi, sigmas, self.deltas.copy())
        else:
            ds = self.ds.copy()
            ds[d_index] -=1
            sigmas = self.sigmas.copy()
            sigmas[option] -= 1
            deltas = self.deltas.copy()
            deltas[option][d_index] += 1
            t = term(self.factor*self.sigmas[option], ds, self.pis, sigmas, deltas)
        return t

    def iterate(self):
        new_terms = []
        if sum(self.ds) == 0 and self.pis == 0:
            self.terminal = True
            new_terms.append(self)
        if self.terminal == False:
            for (i, n) in enumerate(self.ds):
                if n > 0:
                    if self.pis > 0:
                        new_terms.append(self.new_term(i, -1))
                    for (s_i, s_n) in enumerate(self.sigmas):
                        if s_n > 0:
                            new_terms.append(self.new_term(i, s_i))
        return new_terms

    def __str__(self):
        return "factor = {}\nd = {}\npi = {}\nsigma = {}\ndeltas = {}".format(self.factor, self.ds, self.pis, self.sigmas, self.deltas)

def iterate_term_list(a, n):
    term_list = a.copy()
    for i in range(n):
        new_term_list = []
        for t in term_list:
            new_term_list += t.iterate()
        term_list = new_term_list
    return term_list
    
a = [term.initialise(0, 0, 1, 5)]
a = iterate_term_list(a, 9)
