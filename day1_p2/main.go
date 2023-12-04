package main

import (
	"aoc/day1/input"
	"fmt"
	"strings"
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

        // fmt.Printf("%s : %v\n", line, lineval)
    }

    fmt.Printf("Soma total: %d\n", sum)

}

var nums  map[string]uint8 =  map[string]uint8 {

    "one":   1,
    "two":   2,
    "three": 3,
    "four":  4,
    "five":  5,
    "six":   6,
    "seven":  7,
    "eight":  8,
    "nine":   9,
}

func getNumericalValueFromSlice(str string) uint8 {
    
    for k, v := range nums {
       
        if strings.HasPrefix( str, k ) {
            return v
        }
   }

    return 0
}


func isNumerical( b byte ) bool {
    return b >= '0' && b <= '9'
}

const NOT_FOUND uint8 = 10

func generateDigitFromLine( line []byte ) uint8 {
    
    n1, n2 := NOT_FOUND, NOT_FOUND
    sz := len(line)
    
    for i := 0; i < sz; i++ {
    
       if n1 != NOT_FOUND && n2 != NOT_FOUND {
           break
       }

       if n1 == NOT_FOUND {
            
           if isNumerical( line[i] ) {
                n1 = uint8(line[i]) - '0'
           }

           v := getNumericalValueFromSlice( string( line[i:] ) )

           if v != 0 {
                n1 =  v
           }
       }

       if n2 == NOT_FOUND {
            
           if isNumerical( line[sz - i - 1] ) {
                n2 = uint8(line[sz - i - 1]) - '0'
            }

            v := getNumericalValueFromSlice( string( line[(sz - i -1):] ) )

           if v != 0 {
                n2 =  v
           }
 
       }


    }

    return 10 * n1 + n2


}
