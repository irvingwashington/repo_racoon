use std::collections::HashMap;
use crate::recognizer::RepoProperties;

pub type ReposInfo = HashMap<String, RepoProperties>;


// {
//   "Airhelp/ah-cockpit":
//   {
//     "languages": [
//       {"name": "Ruby", "version": "2.4.4", "source": ".ruby-version"}
//     ],
//     "tools": [
//       {"name": "docker", "version": "1.2.3"}
//     ]
//   }
// }
