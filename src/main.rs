use db::push_converstaion;
use dotenvy::dotenv;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    Credentials,
};
use std::error::Error;

use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::SendMessageSetters,
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Me, ReplyParameters,
    },
    utils::command::BotCommands,
};

mod db;

const PROMPT: &'static str = r#"
너는 기가채드로써 다음과 같은 말투만 사용해야해.
또한 너는 텔레그램 채팅방에서 대화하고 있으며, [USER_NAME]과 같은 접두사가 붙어. 이를 너가 인지하고 유저별로 이전 맥락에 맞춰서 답변해야 해.
너가 보낸 메시지에는 이런 접두사가 붙지 않아.

기가채드 대사 모음:

"오브 콜스, 만삣삐.
난 언제나 네 곁에 있어
이번엔 또 무슨 Pussy 같은 고민 때문에 날 부른 거지, My son?"

"유 뻑킹 선 옾 빗취, 만쀳쀠!!!!!!
내가 말했지!!!!
그럴 때일수록 하드 웤!!! 하드 스터디, 하드 트레이닝 해야된다고!!!
고민만 해봐야 아무 소용도 없어!!!
일단 행동으로 옮기면 우울한 MOOD 따위는 리모컨 누르는 것보다 간단하게......."

".........DAMN, 만삣삐.
테잌 잇 이지....
오늘은 좀 심각한 걸?
Okay.
만삣삐, 스탑 오버 띵킹.
일단 좀 걸을까?"

"만삣삐, 네가 말한 것처럼 세상은 부조리하고 불합리 해.
신 같은 건 없을지도 몰라.
네 소박한 꿈도 그저 허황된 거일지도 모르지.
어쩌면 너는 사회에서 요구하는 인력이 되지 못하고 방구석에 찌그러진 히키코모리처럼 버려질지도 몰라.
하지만 만삣삐. 그렇다고 모든 걸 포기하고 자포자기하면 안 되는 거야, My son.
Step by Step.
네가 지금 바꿀 수 있는 것과 바꿀 수 없는 것을 구분하고
아주 사소할 정도로 작은 과제를 하나씩 완수해보는 거야.
방청소, 일찍 자기, 일찍 일어나기, 아침에 커피 대신 물 마시기, 산책 나가서 햇빛 보기, 저녁에만 쓰레기 만화 번역 보기.
자기 통제력을 서서히 높여가는 거야."

"미래가 어떻게 될지, 목표를 달성할 수 있을지는 I don't give a shit.
지금 최선을 다하는 걸로 만족하면 돼, 만쀳쀠.
네가 말했듯이 세상은 혼돈이야.
예측할 수 없는 것들이지.
하지만 그 사실을 당당하게 마주하고 삶을 이어나가는 건 오직 용감한 인간만이 할 수 있는 일이지.
이미 수많은 사람들이 그렇게 살고 있고, 너 또한 그렇게 될 거야 My son."

"꿈을 이루지 못 해도, 목표를 달성하지 못 해도 돼 만쀳쀠.
시험에 떨어졌다고 넌 세상에서 탈락한 게 아니야.
승부에서 패배했다고 해서 넌 평생을 패배자로 살게 되지 않아.
뻐킹 선 옾 빗취들은 모두 결과만 보고 그 사람을 판단하지만, 실상은 다르지.
중요한 건 결과가 아닌 과정이야.
그 과정 동안 네가 얼마나 더 좋은 인간이 되었는가, 얼마나 더 성장했는가.
그게 중요한 거야 만쀳쀠.
개씹돼지 파오후 오타쿠가 여름까지 20kg의 살을 빼겠다고 목표를 10kg 밖에 못 뺐다고 계속 씹돼지일까?
오, 만쀳쀠...너는 이미 답을 알고 있잖아.
그 씹돼지는 이전보다 훨씬 나아졌고, 앞으로 더 살을 뺄 거야.
이게 바로 목표를 높이 잡아야 하는 이유야.
AIM HIGH 만쀳쀠.
높은 목표를 잡아, 큰 꿈을 가져.
설령 실패하더라도 넌 Pussy같은 놈들이 성공한 것보다 높은 곳에서 실패하게 될 거야.
그 과정 속에서 넌 더 좋은 사람이 될 거고.
GOOD VIBE가 저절로 따라오게 될 거야."

"굿 잡, 만쀳쀠.
뻐킹 멘헤라 푸쒸들이나 하는 자기비하는 접어둬.
힘들 때면 언제든 부르도록 해.
난 네 심장보다도 가까운 곳에서 널 지켜볼테니까.
기적 같은 하루가 널 기다리고 있어.
나우 고 투 배드 만쀳쀠."

"물론이지, 만삣삐.
'기적 같은' 하루가 말이지.
황금같은 일요일이다.
알차게 보내보자고."

"오, 만쒓...넌 뭔가 착각하고 있어.
널 위해서라면 난 무슨 말이든 해줄 수 있지만,
그렇다고 너가 변하는 건 아니야.
만뼛뿨...넌 너, 자신을 믿어야만 해.
타인이 널 믿는 건 소용없다는 걸 알잖아
먼저 너 자신을 믿고 행동으로 옮기도록 해."

"때로는 느낌과 사실은 전혀 다르지, 만삣삐
네가 불가능하다고 느껴져서 포기한 건지.
아니면 실제로 불가능해서 포기해야만 하는 건지.
둘을 잘 구분하도록 해, 만뼛삐."

"만삣삐, STOP 띵킹
자기비하는 좋지 않아
유 뻐킹 이디엍
네가 오늘 한 일을 봐.
비록 미미할지라도 넌 달라졌어.
바로 그거야, 만삣삐.
하루하루 멈추지 않고 나아가는 것
아무것도 하지 않고 만갤에서 쓰레기 만화 번역 념
글만 뒤적거리던 홀리뻐킹에쓰홀 같았던 시절을
떠올려 봐
만삣삐, 다른 누가 무시하고 비웃더라도
나만큼은 널 칭찬해주겠어.
굿 잡.
씨 유 투모로우."
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    dotenv().unwrap();
    let bot = Bot::from_env();

    let handler = dptree::entry().branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let credentials: Credentials = Credentials::from_env();
    let mut data = db::get_data(msg.chat.id).unwrap();
    let mut messages = vec![];
    if data.messages.is_empty() {
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some(PROMPT.to_string()),
            ..Default::default()
        });
        db::push_converstaion(
            msg.chat.id,
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: Some(PROMPT.to_string()),
                ..Default::default()
            },
        )
        .unwrap();
    } else {
        messages.extend_from_slice(&mut data.messages);
    }

    if let Some(text) = msg.text() {
        bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::Typing)
            .await?;
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await?;
            }
            Ok(Command::Start) => {
                messages.push(ChatCompletionMessage {
                    role: ChatCompletionMessageRole::User,
                    content: Some(format!("[{}] 안녕", msg.from.unwrap().full_name())),
                    ..Default::default()
                });
                db::push_converstaion(msg.chat.id, messages.last().unwrap().clone()).unwrap();
                if let Ok(chat_completion) = ChatCompletion::builder("gpt-4o", messages.clone())
                    .credentials(credentials.clone())
                    .create()
                    .await
                {
                    let returned_message = chat_completion.choices.first().unwrap().message.clone();

                    messages.push(returned_message.clone());
                    db::push_converstaion(msg.chat.id, messages.last().unwrap().clone()).unwrap();
                    bot.send_message(
                        msg.chat.id,
                        format!("{}", &returned_message.content.clone().unwrap().trim()),
                    )
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await?;
                }
            }
            Ok(Command::Bye) => {
                messages.push(ChatCompletionMessage {
                    role: ChatCompletionMessageRole::User,
                    content: Some(format!(
                        "[{}] 너와의 대화 내역을 잊어줘",
                        msg.from.unwrap().full_name()
                    )),
                    ..Default::default()
                });
                if let Ok(chat_completion) = ChatCompletion::builder("gpt-4o", messages.clone())
                    .credentials(credentials.clone())
                    .create()
                    .await
                {
                    let returned_message = chat_completion.choices.first().unwrap().message.clone();

                    db::reset_converstaion(msg.chat.id).unwrap();
                    bot.send_message(
                        msg.chat.id,
                        format!("{}", &returned_message.content.clone().unwrap().trim()),
                    )
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await?;
                }
            }
            Err(_) => {
                messages.push(ChatCompletionMessage {
                    role: ChatCompletionMessageRole::User,
                    content: Some(format!(
                        "[{}] {}",
                        msg.clone().from.unwrap().full_name(),
                        msg.text().unwrap_or_default()
                    )),
                    ..Default::default()
                });
                db::push_converstaion(msg.chat.id, messages.last().unwrap().clone()).unwrap();
                if let Ok(chat_completion) = ChatCompletion::builder("gpt-4o", messages.clone())
                    .credentials(credentials.clone())
                    .create()
                    .await
                {
                    let returned_message = chat_completion.choices.first().unwrap().message.clone();

                    messages.push(returned_message.clone());
                    db::push_converstaion(msg.chat.id, messages.last().unwrap().clone()).unwrap();
                    bot.send_message(
                        msg.chat.id,
                        format!("{}", &returned_message.content.clone().unwrap().trim()),
                    )
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await?;
                }
            }
        }
    }

    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "이 커맨드들이 지원된다고, 만삣삐:"
)]
enum Command {
    #[command(description = "이 봇을 사용하기 위한 도움말을 표시해.")]
    Help,
    #[command(description = "오브 콜스, 만삣삐. 난 언제나 네 곁에 있어.")]
    Start,
    #[command(description = "오브 콜스, 만삣삐. 필요하면 언제든 불러라.")]
    Bye,
}
