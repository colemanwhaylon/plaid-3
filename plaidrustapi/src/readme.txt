Step #1
git clone https://github.com/plaid/quickstart.git
cd quickstart/node
npm install

Step #2
APP_PORT=8000 \
PLAID_CLIENT_ID= <ENTER YOURS HERE> \
PLAID_SECRET= <ENTER YOURS HERE> \
PLAID_PRODUCTS=transactions \
PLAID_COUNTRY_CODES=US \
PLAID_ENV=sandbox \
node index.js

Step #3
open http://localhost:8000