#tput in action
tput clear
echo "total no. of rows on screen is =\c"
tput lines
echo "total no. of colunms on screen is =\c"
tput cols
tput cup 15 20 
tput bold
echo "this is bold i think"
echo "\033[0mBye Bye"