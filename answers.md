1. En vec som representerar första raden i stdin med nästlade vecs. Sekvenser separerade av whitespaces blir separate Vecs. Nollor representeras med false, och alla andra (icke-whitespace) karaktärer representeras med true.

Exempel. "0100 1001" => \[\[false, true, false, false], \[true, false, false, true\]\]

Exempel. "n0llan" => \[\[true, false, true, true, true, true\]\]

2. a. index_store är en hashmap där keys är siffrorna 1-10, och value = key-1.
2. b. value_store är en Vec av hash sets med längden 1 som innehåller ett tal var. Hela vectorn innehåller talen 1-10.
   Exempel. [{1}, {2}, {3}, {4}, {5}, {6}, {7}, {8}, {9}, {10}]
