#this script helps to create new documents with template
read -p "what type of file do you want to create?" name extn
cp $HOME/Templates/$extn.$extn ./$name.$extn
