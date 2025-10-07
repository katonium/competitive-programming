
setup(){
    __mkdir
}

__mkdir() {
    pwd
    mkdir -p "build/go"
    mkdir -p "build/rust"
    touch input1.txt
    touch input2.txt
    touch input3.txt
    touch main.go
    touch main.rs
}

clean:go(){
    rm -rf build/go/*
}

test:go(){
    clean:go
    go build -o build/go/main main.go
    __test "./build/go/main"
}

clean:rust(){
    rm -rf build/rust/*
}

test:rust(){
    clean:rust
    rustc main.rs -o build/rust/main
    __test "./build/rust/main"
}

__test() {
    cmd="$1"
    echo "$cmd"

    echo "input1"
    echo "==============================="
    cat input1.txt
    echo "==============================="
    cat input1.txt | "$cmd"
    echo ""

    echo "input2"
    echo "==============================="
    cat input2.txt
    echo "==============================="
    cat input2.txt | "$cmd"
    echo ""

    echo "input3"
    echo "==============================="
    cat input3.txt
    echo "==============================="
    cat input3.txt | "$cmd"
    echo ""
}

"$@"
