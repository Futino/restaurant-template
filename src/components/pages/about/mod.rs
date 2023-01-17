use yew::prelude::*;

use crate::components::*;

pub struct About;
impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <main>
          // Top section
          <div
            class="py-40 px-4 sm:px-6 md:px-8 border-b border-secondary-500/40 shadow-2xl">
            <div class="relative max-w-5xl mx-auto text-center">
              <h4 class="text-base font-light text-primary-200 p-4">
                {
                    "Our Story"
                }
            </h4>
            <SeperatorIcon />
            <h1 class=" text-5xl font-normal text-white p-6">
                {"About Us"}
            </h1>
            </div>
          </div>

          // Staff section
          <div class="relative border-b border-secondary-500/40 shadow-2xl">
            <div class="my-40 mx-32 text-center">
              <h1 class="p-6 font-bold text-xl text-primary-200">
                {"WHO WE ARE."}
              </h1>
              <p class="text-">
                {"A modern restaurant with a menu that will make your mouth water. 
                Servicing delicious food since 45 years. Enjoy our seasonal menu and
                 experience the beauty of naturalness"}
              </p>
            </div>
            <div>
              <div id="staff" class="justify-items-center grid grid-cols-2 p-6">
                <StaffCard name="Jonas Lewis"
                        title="Lead Developer, Co-Founder, & Co-Owner" >
                        <p class="text-lg">
                            {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                            ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                            </p>
                            <p class="text-lg">
                                {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis 
                                ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                                </p>
                                <p class="text-lg">
                                  {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                                  ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                                  </p>
                        </StaffCard>
                        <StaffCard name="Jorge Lindberg"
                        title="Lead Developer, Co-Founder, & Co-Owner" >
                        <p>
                            {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                            ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                            </p>
                            <p>
                              {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                              ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                              </p>
                              <p>
                                {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                                ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                                </p>
                        </StaffCard>
              </div>
            </div>
          </div>


          <div class="relative border-b border-secondary-500/40 shadow-2xl">
            <div class="my-32 mx-32">
              <h1 class="inline text-left font-bold text-3xl text-white">
                <a class="font-black text-primary-200">{"What"}</a>
                {" we do."}
              </h1>
              <h1 class="text-left text-white font-normal text-xl max-w-2xl pt-6">
                {"Futino creates and maintains dynamic web-apps that don't rely on
                proprietary subscription-based solutions. We help growing companies
                and startups to build their presence online with beautiful websites
                and apps that they can fully customise."}
              </h1>
            </div>
          </div>
          <div class="relative border-b border-secondary-500/40 shadow-2xl">
            <div class="my-32 mx-32">
              <h1 class="inline text-left font-bold text-3xl text-white">
                <a class="font-black text-primary-200">{"Why"}</a>
                {" we do."}
              </h1>
              <h1 class="text-left text-white font-normal text-xl max-w-2xl pt-6">
                {"Futino creates and maintains dynamic web-apps that don't rely on
                proprietary subscription-based solutions. We help growing companies
                and startups to build their presence online with beautiful websites
                and apps that they can fully customise."}
              </h1>
            </div>
          </div>
        </main>
        }
    }
}
