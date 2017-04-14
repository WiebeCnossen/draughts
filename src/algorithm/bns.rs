/*

function BNS(node, α, β)
 subtreeCount := number of children of node
 do
     test := NextGuess(α, β, subtreeCount)
     betterCount := 0
     foreach child of node
         bestVal := -AlphaBeta(child, -test, -(test - 1))
         if bestVal ≥ test
             betterCount := betterCount + 1
             bestNode := child
     //update number of sub-trees that exceeds separation test value
     //update alpha-beta range
 while not((β - α < 2) or (betterCount = 1))
 return bestNode

 */
