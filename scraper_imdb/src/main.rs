use scraper::{Html, Selector};
use reqwest::Client; 
use reqwest; 
use urlencoding::encode;
use std::io::{self, Write};
use tokio::task; 

struct EpisodeForLookUp{
    title: String,
    season: i32,
    episode: i32,
}   

struct EpisodeOutput{
    episode: EpisodeForLookUp,
    found: bool, 
}

enum SeasonResult {
    Episodes(Vec<EpisodeForLookUp>),
    PageIsNotSeason(bool),
}


// function that itera tes through seasons episodes and calls the charcter search function to see if the character is in the episode
// return a vector that contains the episodes that the character is in
async fn go_through_season(character: &str, url: &str, season: i32, client: &Client) ->  Result<SeasonResult, Box<dyn std::error::Error>> {
    let season_url = format!("{}{}", url, season); 
    let season_response = client.get(&season_url).send().await?;
    let season_html = season_response.text().await?;
    let season_document = Html::parse_document(&season_html);

    // check if the page include a specific p tag that specifies if the page is an invalid season meaning the season does not exist
    // it looks like we don't have any episode list for this title yet. 
    let invalid_season_selector = Selector::parse("p").unwrap(); 
    

    if let Some(invalid_season) = season_document.select(&invalid_season_selector).next(){
        if invalid_season.text().collect::<Vec<_>>().join(" ") == "It looks like we don't have any episode list for this title yet.   Be the first to contribute."{
            return Ok(SeasonResult::PageIsNotSeason(true));
        }
    }

    let section_of_episode = Selector::parse("section.sc-e55007c4-0").unwrap();
    let article_selector = Selector::parse("article").unwrap(); 
    let link_selector = Selector::parse("a").unwrap(); 


    


    //class="sc-e55007c4-0 fZcppP"
    // iterate through the episodes which are article tags in the section tag 
    // then get the link to the episode page and use it to call the go_through_episode function
    // if the character is in the episode the go_through_episode_function will return a struct with the episode information and true
    // then we can safley push it to the vector of episodes that the characer is in
    let mut episode_vector = Vec::new();
    if let Some(section_selector) = season_document.select(&section_of_episode).next() {
        for article in section_selector.select(&article_selector){
            
            for link_element in article.select(&link_selector){

                if let Some(href) = link_element.value().attr("href"){
                   
                    // pass the new link through the go_through_episode function
                    let episode = go_through_episode(character, season, href, client).await;
                    match episode{
                        Ok(episode) => {
                            if episode.found{   
                                episode_vector.push(episode.episode);

                            }
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            }


        }
    }
    Ok(SeasonResult::Episodes(episode_vector))

    
}

async fn go_through_episode(character: &str, season: i32 ,url: &str, client: &Client) -> Result<EpisodeOutput, Box<dyn std::error::Error>>{

    // need to add the prefix to the url 
    
    // get the last index of the url to get the episode number than cast this to a int
    // meaning the last index ofrf the url the last character in it
    // not the last /
    // THE LAST CHARACTER IN THE URL
    let mut episode : i32 = 0; 
    if let Some(last_char) = url.chars().last(){
        // cast last_char as an int
        let last_char_as_string = last_char.to_string();
        match last_char_as_string.parse::<i32>(){
            // if the last character is a number then set the episode number to that number
            Ok(num) => {
                episode = num;
            }
            // if the last character is not a number then set the episode number to 0
            Err(_) => {
                episode = 0;
            }
        }
    } else{
        println!("the string is empty so error"); 
    }



    let url = format!("https://www.imdb.com{}", url);
    let episode_response = client.get(url).send().await?; 

    let episode_html = episode_response.text().await?;
    let episode_document = Html::parse_document(&episode_html); 
    let cast_link_selector = Selector::parse("a.ipc-title-link-wrapper").unwrap(); 
    let main_selector = Selector::parse("main").unwrap(); 
    let section_selector = Selector::parse("section.ipc-page-section").unwrap(); 
    let top_cast_div_selector = Selector::parse("div.ipc-title__wrapper").unwrap();
    // grab the selector to get to the cast list page
    if let Some(section) = episode_document.select(&top_cast_div_selector).nth(1){
        if let Some(cast_link) = section.select(&cast_link_selector).next(){

            if let Some(href) = cast_link.value().attr("href"){
                // go to the link
                let u = format!("https://www.imdb.com{}", href);
                let cast_response = client.get(u.clone()).send().await?;
                if !cast_response.status().is_success(){
                    println!("Error, Failed to catch the cast page, {}", cast_response.status());
                } 
                println!("URL: {}", u.clone());
                
                let cast_html = cast_response.text().await?;
                let cast_document = Html::parse_document(&cast_html);
                let cast_table_selector = Selector::parse("table.cast_list").unwrap();
                let cast_table_body_selector = Selector::parse("tbody").unwrap(); 
                let cast_table_row_selector = Selector::parse("tr").unwrap(); 
                let cast_table_character_selector = Selector::parse("td.character").unwrap(); 
                println!("Before the cast table"); 
                let header_3_selector = Selector::parse("h3").unwrap(); 
                let episode_title_selector = Selector::parse("a").unwrap(); 
                let mut title = String::new();
                if let Some(header_3) = cast_document.select(&header_3_selector).next(){
                    if let Some(episode_title) = header_3.select(&episode_title_selector).next(){
                        title = episode_title.text().collect::<Vec<_>>().join(" "); 

                    }
                }


                if let Some(cast_table) = cast_document.select(&cast_table_selector).next(){
                
                    if let Some(cast_table_body) = cast_table.select(&cast_table_body_selector).next(){
                        
                        // iterate through each row in the body of the table and select the character entry
                        for cast_table_row in cast_table_body.select(&cast_table_row_selector){
                            
                            if let Some(cast_table_character) = cast_table_row.select(&cast_table_character_selector).next(){
                                if cast_table_character.text().collect::<Vec<_>>().join(" ").contains(character){
                                    // if the character is in the episode return the episode information and true
                                    let episode = EpisodeForLookUp{
                                        title: title.to_string(),
                                        season: season,
                                        episode: episode,
                                    }; 
                                    return Ok(EpisodeOutput{episode, found: true});
                                }
                            }

                        }

                    }

                }
            }
        }
    }
    Ok(EpisodeOutput {
        episode: EpisodeForLookUp {
            title: "Unknown".to_string(),
            season: 0,
            episode: 0,
        },
        found: false,
    })
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the user input and create search URL
    let mut search_input = String::new();
    let mut character_input = String::new(); 
    print!("Enter a show: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut search_input)?;
    let search_input = search_input.trim();

    print!("Enter a character to search for: ");

    io::stdout().flush()?;
    io::stdin().read_line(&mut character_input)?;
    let character_input = character_input.trim();

    // URL encode the input
    let encoded_input = encode(search_input);

    // Construct the search URL
    let search_url = format!("https://www.imdb.com/find/?q={}&ref_=nv_sr_sm", encoded_input);
    
    // Print the search URL for debugging
    println!("{}", search_url);

    // Perform the HTTP GET request
    let client = Client::new();
    let response = client.get(&search_url).send().await?;
    let html_content = response.text().await?;

    // Get the episode page URL
    if let Some(episodes_url) = find_episode_page_url(&html_content, &client).await? {

        // Now, you can scrape the Episodes page
        let episodes_response = client.get(&episodes_url).send().await?;
        let episodes_html = episodes_response.text().await?;


        // Parse the episodes page
        let episodes_document = Html::parse_document(&episodes_html);

        
        let mut episode_vector = Vec::new();
        if let Some(index) = episodes_url.rfind('/') {
            let mut season_url = &episodes_url[..index];
            let better_url = format!("{}?season=", season_url);

            // iterate through the seasons pages
            // a season page for a season that does not exist will return a page
            let mut season = 1; 
            // loop through 
            loop{
                let season_result = go_through_season(character_input, &better_url, season, &client); 
                match season_result.await{
                    // append the season result vector to the episode vector
                    Ok(SeasonResult::Episodes(mut episodes))=> {
                        let result = 
                        episode_vector.append(&mut episodes);
                        season += 1;

                    } 

                    Ok(SeasonResult::PageIsNotSeason(false)) => {
                        println!("Valid season page, but no episodes found.");
                        season += 1; 
                    }
                    // if the page is not a season page then break the loop
                    Ok(SeasonResult::PageIsNotSeason(true)) => {
                        break;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }
            // print the items in the vector
            for episode in episode_vector{
                println!("{}: Season {}, Episode {}", episode.title, episode.season, episode.episode);
            }

        
            
        } 
    } else {
        println!("Episodes page not found.");
    }

    



    Ok(())
}

// Function to find the episode page URL
async fn find_episode_page_url(html_content: &str, client: &Client) -> Result<Option<String>, Box<dyn std::error::Error>> {
    // Parse the HTML content
    let document = Html::parse_document(html_content);
    
    // Select the first <ul> with all necessary classes
    let ul_selector = Selector::parse("ul.ipc-metadata-list.ipc-metadata-list--dividers-after.sc-e8e4ce7-3.dTEYDP")?;
    let ul_element = match document.select(&ul_selector).next() {
        Some(ul) => ul,
        None => return Ok(None),
    };
    
    // Get the first <li> element
    let first_li = match ul_element.select(&Selector::parse("li")?).next() {
        Some(li) => li,
        None => return Ok(None),
    };
    
    // Find the <a> tag within the first <li> to get the link
    let link_element = match first_li.select(&Selector::parse("a")?).next() {
        Some(a) => a,
        None => return Ok(None),
    };
    
    // Extract the href attribute (link to the next page)
    let href = match link_element.value().attr("href") {
        Some(href) => href,
        None => return Ok(None),
    };
    
    // Construct the full URL to the page
    let full_url = format!("https://www.imdb.com{}", href);

    // Now, make a request to the new URL (Detail page)
    let detail_response = client.get(&full_url).send().await?;
    let detail_html = detail_response.text().await?;

    // Parse the detail page
    let detail_document = Html::parse_document(&detail_html);
    
    // Find the link to the "Episodes" page
    let episodes_selector = Selector::parse("a[href*='/episodes']")?;
    let episodes_link = match detail_document.select(&episodes_selector).next() {
        Some(link) => link,
        None => return Ok(None),
    };

    let episodes_href = match episodes_link.value().attr("href") {
        Some(href) => href,
        None => return Ok(None),
    };

    // Construct the full URL to the Episodes page
    let episodes_url = format!("https://www.imdb.com{}", episodes_href);
    Ok(Some(episodes_url))
}
