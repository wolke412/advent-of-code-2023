package input

import (
	"bufio"
	"log"
	"os"
)

const (

    SOURCE_FILES_FOLDER = "input"
)

func GetBuffer( file_name string ) *bufio.Reader {
    
    file, err := os.Open( SOURCE_FILES_FOLDER + "/" + file_name )

    if err != nil {
        log.Fatal("Erro abrindo arquivo")
    }

    r := bufio.NewReader(file)

    return r
}
