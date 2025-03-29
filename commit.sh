git add .
echo "Added everything to commit"
read -p "Commit name - " name
read -p "Commit desc - " desc
git commit -m "$name" -m "$desc"
echo "Commited changes!"
git push
echo Pushed!
