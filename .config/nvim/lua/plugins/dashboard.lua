return {
  "folke/snacks.nvim",
  opts = {
    dashboard = {
      preset = {
        header = [[                                                         
                                                    ___  
                                                 ,o88888 
                                              ,o8888888' 
                        ,:o:o:oooo.        ,8O88Pd8888"  
                    ,.::.::o:ooooOoOoO. ,oO8O8Pd888'"    
                  ,.:.::o:ooOoOoOO8O8OOo.8OOPd8O8O"      
                 , ..:.::o:ooOoOOOO8OOOOo.FdO8O8"        
                , ..:.::o:ooOoOO8O888O8O,COCOO"          
               , . ..:.::o:ooOoOOOO8OOOOCOCO"            
                . ..:.::o:ooOoOoOO8O8OCCCC"o             
                   . ..:.::o:ooooOoCoCCC"o:o             
                   . ..:.::o:o:,cooooCo"oo:o:            
                `   . . ..:.:cocoooo"'o:o:::'            
                .`   . ..::ccccoc"'o:o:o:::'             
               :.:.    ,c:cccc"':.:.:.:.:.'              
             ..:.:"'`::::c:"'..:.:.:.:.:.'               
           ...:.'.:.::::"'    . . . . .'                 
          .. . ....:."' `   .  . . ''                    
        . . . ...."'                                     
        .. . ."'                                         
       .                                                 
                                                         ]],

        keys = {
          { icon = "󰉋 ", key = "f", desc = "Find File", action = ":lua Snacks.dashboard.pick('files')" },
          { icon = " ", key = "n", desc = "New File", action = ":ene | startinsert" },
          { icon = " ", key = "p", desc = "Projects", action = ":lua Snacks.dashboard.pick('projects')" },
          { icon = "󰊄 ", key = "g", desc = "Find Text", action = ":lua Snacks.dashboard.pick('live_grep')" },
          { icon = "󰥔 ", key = "r", desc = "Recent Files", action = ":lua Snacks.dashboard.pick('oldfiles')" },
          {
            icon = " ",
            key = "c",
            desc = "Config",
            action = ":lua Snacks.dashboard.pick('files', {cwd = vim.fn.stdpath('config')})",
          },
          { icon = " ", key = "s", desc = "Restore Session", action = ":lua require('persistence').load()" },
          { icon = "󰅚 ", key = "q", desc = "Quit", action = ":qa" },
        },
      },
    },
  },
}
