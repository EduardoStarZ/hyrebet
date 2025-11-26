#!/bin/sh

wrk -c 10 -d 5m -t 4 -H --latency http://127.0.0.1:4000/login > test-results/5m-login-rf.txt

wrk -c 10 -d 5m -t 4 -H --latency http://127.0.0.1:5000/login > test-results/5m-login-rl.txt

wrk -c 10 -d 5m -t 4 -H --latency http://127.0.0.1:4000/register > test-results/5m-register-rf.txt

wrk -c 10 -d 5m -t 4 -H --latency http://127.0.0.1:5000/ > test-results/5m-register-rl.txt

wrk -c 10 -d 5m -t 4 -H "Cookie: Auth=Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJlZHVhcmRvIiwiZXhwIjoxNzY0ODU1Mjk3fQ.M_OsPdEWkTbvTASTCUpiC-LLS8AYLna-a3pW7OTaVxI" --latency http://127.0.0.1:4000/ > test-results/5m-home-rf.txt

wrk -c 10 -d 5m -t 4 -H "Cookie: Auth=Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJlZHVhcmRvIiwiZXhwIjoxNzY0ODU1Mjk3fQ.M_OsPdEWkTbvTASTCUpiC-LLS8AYLna-a3pW7OTaVxI" --latency http://127.0.0.1:5000/home > test-results/5m-home-rl.txt

wrk -c 10 -d 5m -t 4 -H "Cookie: Auth=Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJlZHVhcmRvIiwiZXhwIjoxNzY0ODU1Mjk3fQ.M_OsPdEWkTbvTASTCUpiC-LLS8AYLna-a3pW7OTaVxI" --latency http://127.0.0.1:4000/eduardo > test-results/5m-profile-rf.txt

wrk -c 10 -d 5m -t 4 -H "Cookie: Auth=Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJlZHVhcmRvIiwiZXhwIjoxNzY0ODU1Mjk3fQ.M_OsPdEWkTbvTASTCUpiC-LLS8AYLna-a3pW7OTaVxI" --latency http://127.0.0.1:5000/eduardo > test-results/5m-profile-rl.txt

wrk -c 10 -d 5m -t 4 -H "Cookie: Auth=Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJlZHVhcmRvIiwiZXhwIjoxNzY0ODU1Mjk3fQ.M_OsPdEWkTbvTASTCUpiC-LLS8AYLna-a3pW7OTaVxI" --latency http://127.0.0.1:4000/eduardo/104924087?fullview=true > test-results/5m-post-rf.txt

wrk -c 10 -d 5m -t 4 -H "Cookie: Auth=Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJlZHVhcmRvIiwiZXhwIjoxNzY0ODU1Mjk3fQ.M_OsPdEWkTbvTASTCUpiC-LLS8AYLna-a3pW7OTaVxI" --latency http://127.0.0.1:5000/eduardo/104924087 > test-results/5m-post-rl.txt
