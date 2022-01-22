use crate::network;
use anyhow::Result;
use rand::{prelude::*, thread_rng, Rng};
use yoga_forum::entities::{Comment, Story};

pub async fn run_actor(id: usize) -> Result<()> {
    let mut state = State::LoggedOutNew;
    loop {
        let r: u8 = {
            let mut rng = thread_rng();
            rng.gen_range(0..10)
        };
        state = state.next(id, r).await?;
    }
}

enum State {
    LoggedOutNew,
    LoggedOutViewStories {
        stories: Vec<Story>,
        next_token: Option<String>,
    },
    LoggedOutViewStory {
        story: Story,
        _comments: Vec<Comment>,
        next_token: Option<String>,
    },
    LoggedInNew {
        login_token: String,
    },
    LoggedInViewStories {
        login_token: String,
        stories: Vec<Story>,
        next_token: Option<String>,
    },
    LoggedInViewStoryNew {
        login_token: String,
        story: Story,
    },
    LoggedInViewStory {
        login_token: String,
        story: Story,
        comments: Vec<Comment>,
        next_token: Option<String>,
    },
}

impl State {
    pub async fn next(self, id: usize, rng: u8) -> Result<State> {
        assert!(rng < 10);
        use State::*;
        Ok(match (self, rng) {
            // Initialize with some stories
            (LoggedOutNew, _) => {
                println!("{id}: Logged out -> New");
                let mut res = network::load_stories(None).await?;
                let next_token = res.next_token.take();
                let stories = res.stories;
                assert!(stories.len() > 0);
                LoggedOutViewStories {
                    next_token,
                    stories,
                }
            }

            /* Logged Out & View Stories */
            // View Next Page
            (LoggedOutViewStories { next_token, .. }, 0 | 1 | 2) => {
                println!("{id}: Logged out -> Next Page of Stories");
                let mut res = network::load_stories(next_token.as_deref()).await?;
                let next_token = res.next_token.take();
                let stories = res.stories;
                LoggedOutViewStories {
                    next_token,
                    stories,
                }
            }

            // View A Story
            (LoggedOutViewStories { mut stories, .. }, 3 | 4 | 5) => {
                println!("{id}: Logged out -> View a story");
                let story: Story = {
                    let mut rng = thread_rng();
                    stories.drain(..).choose(&mut rng).unwrap()
                };
                let mut res = network::load_comments(story.id, None).await?;
                let next_token = res.next_token.take();
                let _comments = res.comments;
                LoggedOutViewStory {
                    story,
                    next_token,
                    _comments,
                }
            }

            // Log in
            (
                LoggedOutViewStories {
                    stories,
                    next_token,
                },
                6 | 7 | 8 | 9,
            ) => {
                println!("{id}: Logged out -> Logging in");
                let user_id = network::create_user().await?;
                let login_token = network::sign_in(user_id).await?;
                LoggedInViewStories {
                    login_token,
                    stories,
                    next_token,
                }
            }

            /* Logged Out & View Story */
            // Load more comments
            (
                LoggedOutViewStory {
                    story, next_token, ..
                },
                0 | 1 | 2 | 3,
            ) => {
                println!("{id}: Logged out -> Load more comments");
                let mut res = network::load_comments(story.id, next_token.as_deref()).await?;
                let next_token = res.next_token.take();
                let _comments = res.comments;
                LoggedOutViewStory {
                    story,
                    _comments,
                    next_token,
                }
            }

            // Back out to story page
            (LoggedOutViewStory { .. }, _) => {
                println!("{id}: Logged out -> Back to stories");
                LoggedOutNew
            }

            /* Logged In & New */
            // Initialize with some stories
            (LoggedInNew { login_token }, _) => {
                println!("{id}: Logged in -> New");
                let mut res = network::load_stories(None).await?;
                let next_token = res.next_token.take();
                let stories = res.stories;
                assert!(stories.len() > 0);
                LoggedInViewStories {
                    login_token,
                    next_token,
                    stories,
                }
            }

            /* Logged In & View Stories */
            // View Next Page
            (
                LoggedInViewStories {
                    login_token,
                    next_token,
                    ..
                },
                0 | 1 | 2,
            ) => {
                println!("{id}: Logged in -> Next page of stories");
                let mut res = network::load_stories(next_token.as_deref()).await?;
                let next_token = res.next_token.take();
                let stories = res.stories;
                LoggedInViewStories {
                    login_token,
                    next_token,
                    stories,
                }
            }

            // Create a new story
            (LoggedInViewStories { login_token, .. }, 3) => {
                println!("{id}: Logged in -> Create a story");
                network::create_story(&login_token).await?;
                LoggedInNew { login_token }
            }

            // View a story
            (
                LoggedInViewStories {
                    login_token,
                    mut stories,
                    ..
                },
                4 | 5 | 6 | 7 | 8,
            ) => {
                println!("{id}: Logged in -> View a story");
                let story: Story = {
                    let mut rng = thread_rng();
                    stories.drain(..).choose(&mut rng).unwrap()
                };
                LoggedInViewStoryNew { login_token, story }
            }

            // Log out
            (LoggedInViewStories { .. }, 9) => {
                println!("{id}: Logged in -> Logging out");
                LoggedOutNew
            }

            /* Logged In & View Story NEW */
            (LoggedInViewStoryNew { login_token, story }, _) => {
                println!("{id}: Logged in -> Loading comments");
                let mut res = network::load_comments(story.id, None).await?;
                let next_token = res.next_token.take();
                let comments = res.comments;
                LoggedInViewStory {
                    login_token,
                    next_token,
                    story,
                    comments,
                }
            }

            /* Logged In & View Story */
            // Load more comments
            (
                LoggedInViewStory {
                    login_token,
                    story,
                    next_token,
                    ..
                },
                0 | 1 | 2,
            ) => {
                println!("{id}: Logged in -> Loading more comments");
                let mut res = network::load_comments(story.id, next_token.as_deref()).await?;
                let next_token = res.next_token.take();
                let comments = res.comments;
                LoggedInViewStory {
                    login_token,
                    story,
                    comments,
                    next_token,
                }
            }

            // Create a comment
            (
                LoggedInViewStory {
                    login_token,
                    story,
                    next_token,
                    comments,
                },
                3 | 4,
            ) => {
                println!("{id}: Logged in -> Creating a comment");
                network::create_comment(&login_token, story.id, None).await?;
                LoggedInViewStory {
                    login_token,
                    story,
                    comments,
                    next_token,
                }
            }

            // Respond to a comment
            (
                LoggedInViewStory {
                    login_token,
                    story,
                    mut comments,
                    ..
                },
                5 | 6 | 7 | 8,
            ) => {
                println!("{id}: Logged in -> Responding to a comment");
                if comments.is_empty() {
                    // TODO: Maybe something better than just skipping?
                    LoggedInViewStoryNew { login_token, story }
                } else {
                    let parent: Comment = {
                        let mut rng = thread_rng();
                        comments.drain(..).choose(&mut rng).unwrap()
                    };
                    network::create_comment(&login_token, story.id, Some(parent.id)).await?;
                    LoggedInViewStoryNew { login_token, story }
                }
            }

            // Back out to stories
            (LoggedInViewStory { login_token, .. }, 9) => {
                println!("{id}: Logged in -> Back to stories");
                LoggedInNew { login_token }
            }

            _ => panic!("Invalid state reached"),
        })
    }
}
