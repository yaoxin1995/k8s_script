#!/bin/bash
set -e



log_path="`pwd`"
filename=docker_cleanup_$timestamp.log
log=$log_path/$filename
timestamp=$(date +%Y%m%d_%H%M%S)



docker_find (){
    echo "#####################################################################" >> $log
    echo "Finding syscall-test images" >> $log
    echo "#####################################################################" >> $log
    REMOVEIMAGES=`docker images | grep "syscall-test" | awk '{print $3}'`

    echo "Listing images that needs to be cleaned up" >> $log
    echo $REMOVEIMAGES >>$log
}


docker_cleanup(){
    echo "#####################################################################" >> $log
    echo "Cleaning images" >> $log
    echo "#####################################################################" >> $log
    docker rmi -f ${REMOVEIMAGES}
}


docker_space_after(){
    CURRENTSPACE=`docker system df`
    echo "Current Docker Space, after clean up:" >> $log
    echo $CURRENTSPACE >>$log
}


# docker_space_before
# docker_find
# docker_cleanup
# docker_space_after



# docker build -t  syscall-test .

# sudo docker tag syscall-test:latest yaoxinjing/syscall-test:latest
# sudo docker push yaoxinjing/syscall-test


# docker build -t  syscall-inteceptor-test .

# sudo docker tag  syscall-inteceptor-test:latest yaoxinjing/syscall-inteceptor-test:latest
# sudo docker push yaoxinjing/syscall-inteceptor-test


# sequencial write
# micro-bench-elf-1024mb
# micro-bench-elf-100mb
# micro-bench-elf-10mb
# micro-bench-elf-5mb


# sequencial read
# micro-bench-elf-5mb-sr
# yaoxinjing/micro-bench-elf-10mb-sr:latest
# micro-bench-elf-100mb-sr
# micro-bench-elf-1024mb-sr

# cquark-sequencial-read
# duration 6994580
# duration 8227992
# duration 87284018
# duration 811623739

# baseline-sequencial-read
# duration 13247999
# duration 17716737
# duration 183102544
# duration 1417822475

# cquark-sequencial-write
# duration x
# duration x
# duration x
# duration x

# baseline-sequencial-write
# duration x
# duration x
# duration x
# duration x


# cquark-random-read
# duration 6576042484
# duration 13110102153
# duration 143272391801

# baseline-random-read
# duration 6595378628
# duration 13112227333
# duration 143161966077



# cquark-random-write
# duration 6595744379
# duration 13134447259
# duration 130062098555

# baseline-random-write
# duration 6599000362
# duration 13243437992
# duration 130845013559





# randome read
# micro-bench-elf-1024mb-rr
# micro-bench-elf-100mb-rr 
# micro-bench-elf-10mb-rr
# micro-bench-elf-5mb-rr


#  random write
# micro-bench-elf-1024mb-rw
# micro-bench-elf-100mb-rw
#  micro-bench-elf-10mb-rw
#  micro-bench-elf-5mb-rw


# micro-bench-pasue 


make 

sudo docker build -t   micro-bench-elf-100mb-rw .

sudo docker tag   micro-bench-elf-100mb-rw:latest yaoxinjing/micro-bench-elf-100mb-rw:latest
sudo docker push yaoxinjing/micro-bench-elf-100mb-rw







