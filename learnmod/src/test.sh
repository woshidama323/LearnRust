#!/bin/bash

INPUTDIR=/data/car/source
MaxUsed=90
ForCarFiles=/data/car/downloaded.txt
CarGenerateBin=/data/car/create_car
## 确保当前的目标目录size足够
targetDIRList="data8 data7 data6 data5 data4"
IFS=', ' read -r -a outList <<< $targetDIRList

### 默认是data8
TargetDir=data8

function SetTargetOutDir() 
{
	echo "starting set SetTargetOutDir...."
	tmp=TargetDir
	curUsePercent=`df -BM /$TargetDir --output=pcent|grep -v Use|awk -F% '{print $1}'`
	### 当前目录使用率>MaxUsed 则重新选择一个新的目录
	if [[ $curUsePercent -ge $MaxUsed ]];then
		for d in $targetDIRList
		do
			targetUsePercent=`df -BM /$d --output=pcent|grep -v Use|awk -F% '{print $1}'`
			if [[ $targetUsePercent -le $MaxUsed ]];then
				TargetDir=d
				echo "get new target data dir $d"
				break
			fi
		done
		if [[ "$TargetDir" == "$tmp" ]];then
			echo "no enought dir to be used.."
		fi
	else
		echo "using target is $TargetDir"
	fi
}

for i in `cat $ForCarFiles`
do
	echo "......start generate car file"
	SetTargetOutDir
	$CarGenerateBin -input-path $INPUTDIR/$i -out-dir /$TargetDir/car/ >> creat_car.log 2>&1 
	if [[ $? -ne 0 ]];then
		echo "failed to create car files for $INPUTDIR/$i"
		tail -5 creat_car.log
	else
		## 成功打完car 删除原始文件
		echo "succeed in creating car for $INPUTDIR/$i, start remove origin file"
		rm $INPUTDIR/$i >/dev/null 2>&1
		if [[ $? -ne 0 ]];then
    	echo "failed to remove file $INPUTDIR/$i"
    fi
	fi
done

echo "Have finished all the files for creating car...."