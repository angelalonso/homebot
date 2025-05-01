Code architecture Scheme

                       ./home.sh
                          | - Identifies mode
                          | - Builds
                          | - Installs
                          | - Runs/Stops
   ===========================================================
                          V
                        main
                          | - Loads config file
   --- test, live --------|----------- sim ----------
   V                                                V
  env                                             sim_env
   | - Runs "run" function
   | - "run" is the loop
   | - TODO: manage properly input
   V 
