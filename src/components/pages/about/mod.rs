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
            class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-secondary-500/40">
            <div class="relative mx-auto max-w-5xl text-center">
              <h4 class="p-4 text-base font-light text-primary-200">
                {
                    "Our Story"
                }
            </h4>
            <SeperatorIcon />
            <h1 class="p-6 text-5xl font-normal text-white">
                {"About Us"}
            </h1>
            </div>
          </div>

          // Staff section
          <div class="relative border-b shadow-2xl border-secondary-500/40">
            <div class="my-40 mx-32 text-center">
              <h1 class="p-6 text-xl font-bold text-primary-200">
                {"WHO WE ARE."}
              </h1>
              <p class="text-">
                {"A modern restaurant with a menu that will make your mouth water. 
                Servicing delicious food since 45 years. Enjoy our seasonal menu and
                 experience the beauty of naturalness"}
              </p>
            </div>
            <div>
              <div id="staff" class="grid grid-cols-2 justify-items-center p-6">
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


          <div class="relative border-b shadow-2xl border-secondary-500/40">
            <div class="my-32 mx-32">
              <h1 class="inline text-3xl font-bold text-left text-white">
                <a class="font-black text-primary-200">{"What"}</a>
                {" we do."}
              </h1>
              <h1 class="pt-6 max-w-2xl text-xl font-normal text-left text-white">
                {"Futino creates and maintains dynamic web-apps that don't rely on
                proprietary subscription-based solutions. We help growing companies
                and startups to build their presence online with beautiful websites
                and apps that they can fully customise."}
              </h1>
            </div>
          </div>
          <div class="relative border-b shadow-2xl border-secondary-500/40">
            <div class="my-32 mx-32">
              <h1 class="inline text-3xl font-bold text-left text-white">
                <a class="font-black text-primary-200">{"Why"}</a>
                {" we do."}
              </h1>
              <h1 class="pt-6 max-w-2xl text-xl font-normal text-left text-white">
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
