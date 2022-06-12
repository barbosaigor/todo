## TODO
ToDo searches for all `TODO:`s in your current directory and report them.  

### Install
```sh
$ cargo install todo
```  

### Usage  
```sh
$ todo <DIR> <FLAGS> 
```

```sh
$ todo
./internal/domain/service/orchestrator/orchestrator.go:114 -> tweak, should be configurable
./vendor/github.com/barbosaigor/hey/you.go:82 -> add support for optional groups "/hey(/there)?"
./vendor/github.com/barbosaigor/hey/you.go:112 -> add support for optional groups "/abc(/def)?"
./vendor/github.com/barbosaigor/hey/you.go:120 -> add support for optional groups "/bc?"
```

By default todo will use the current directory if DIR is not provided.  
To exclude folders/files that matches a substring use flag **--exclude**  
```sh
$ todo --exclude=vendor
./internal/domain/service/orchestrator/orchestrator.go:114 -> tweak, should be configurable
```  