search=""
while read line; do search=$search$line; done 
echo $search
google-chrome google.com/search?q=$search