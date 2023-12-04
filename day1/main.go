package main

import (
	"aoc/day1/input"
	"fmt"

)

func main () {


    reader := input.GetBuffer( "input.txt" )
    sum := uint32(0)

    for {

        line, _, err := reader.ReadLine() 
        
        if err != nil {
            break
        }

        lineval := generateDigitFromLine( line )
        sum += uint32(lineval)   
    }

    fmt.Printf("Soma total: %d\n", sum)

}

func isNumerical( b byte ) bool {
    return b >= '0' && b <= '9'
}

func generateDigitFromLine( line []byte ) uint8 {
    
    var n1, n2 uint8
  
    sz := len(line)
    found := 0b0
    
    for i := 0; i < sz; i++ {
    
       if found == 0b11 {
           break
       }

       if found & 0b1 == 0 {
            if isNumerical( line[i] ) {
                n1 = uint8(line[i]) - '0'
                found |= 0b1
            }
       }

       if found & 0b10 == 0 {
            if isNumerical( line[sz - i - 1] ) {
                n2 = uint8(line[sz - i - 1]) - '0'
                found |= 0b10
            }
       }

    }

    return 10 * n1 + n2


}
