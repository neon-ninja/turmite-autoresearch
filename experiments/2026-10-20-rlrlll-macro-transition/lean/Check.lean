import Model
import Iter
import Data
/-! `lake env lean Check.lean` prints the axioms every theorem used. Expected: only the core axioms (propext, Quot.sound, ...),
    no `sorryAx`, no `Lean.ofReduceBool` (that one would mean `native_decide`). -/
open Sweeper
#print axioms runN_emb
#print axioms descent_iter
#print axioms return_iter
#print axioms descent_cert0
#print axioms descent_cert1
#print axioms descent_cert2
#print axioms return_cert0
#print axioms return_cert1
#print axioms return_cert2
#print axioms descent_all0
#print axioms descent_all1
#print axioms descent_all2
#print axioms return_all0
#print axioms return_all1
#print axioms return_all2
