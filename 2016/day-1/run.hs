import qualified Data.Text as T

type Pos = (Integer, Integer)
type Rot = Integer
type Steps = Integer
type Dist = Integer
type Instr = String
type Input = String

instrToRot :: Instr -> Rot -> Rot
instrToRot instr currRot =
  case head instr of
    'R' -> currRot + 1
    'L' -> currRot - 1

instrToSteps :: Instr -> Steps
instrToSteps instr = read $ tail instr

rotAndStepsToDeltaPos :: Rot -> Steps -> Pos
rotAndStepsToDeltaPos rot steps =
  case rot `mod` 4 of
    0 -> (0, steps)
    1 -> (steps, 0)
    2 -> (0, -steps)
    3 -> (-steps, 0)

posToDist :: Pos -> Dist
posToDist (x, y) = abs x + abs y

splitInput :: Input -> [Instr]
splitInput input = map T.unpack $ T.splitOn (T.pack ", ") (T.pack input)

instrsToPos :: [Instr] -> Pos
instrsToPos instrs = snd $ foldl prevStateToNextState (0, (0, 0)) instrs

instrsToDist instrs = posToDist $ instrsToPos instrs

type State = (Rot, Pos)

prevStateToNextState :: State -> Instr -> State
prevStateToNextState (prevRot, (prevX, prevY)) instr =
  (nextRot, (prevX + deltaX, prevY + deltaY))
    where nextRot = instrToRot instr prevRot
          nextSteps = instrToSteps instr
          (deltaX, deltaY) = rotAndStepsToDeltaPos nextRot nextSteps

inputInstrs = "R4, R5, L5, L5, L3, R2, R1, R1, L5, R5, R2, L1, L3, L4, R3, L1, L1, R2, R3, R3, R1, L3, L5, R3, R1, L1, R1, R2, L1, L4, L5, R4, R2, L192, R5, L2, R53, R1, L5, R73, R5, L5, R186, L3, L2, R1, R3, L3, L3, R1, L4, L2, R3, L5, R4, R3, R1, L1, R5, R2, R1, R1, R1, R3, R2, L1, R5, R1, L5, R2, L2, L4, R3, L1, R4, L5, R4, R3, L5, L3, R4, R2, L5, L5, R2, R3, R5, R4, R2, R1, L1, L5, L2, L3, L4, L5, L4, L5, L1, R3, R4, R5, R3, L5, L4, L3, L1, L4, R2, R5, R5, R4, L2, L4, R3, R1, L2, R5, L5, R1, R1, L1, L5, L5, L2, L1, R5, R2, L4, L1, R4, R3, L3, R1, R5, L1, L4, R2, L3, R5, R3, R1, L3"

main = putStrLn $ show $ instrsToDist $ splitInput $ inputInstrs