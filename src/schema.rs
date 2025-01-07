// // @generated automatically by Diesel CLI.

// diesel::table! {
//     activities (id) {
//         id -> Int4,
//         date -> Nullable<Date>,
//         #[max_length = 255]
//         title -> Nullable<Varchar>,
//         description -> Nullable<Text>,
//         start -> Nullable<Time>,
//         end -> Nullable<Time>,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         #[max_length = 200]
//         category_id -> Nullable<Varchar>,
//         journal_id -> Nullable<Int4>,
//         user_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     api_advertisement (id) {
//         id -> Int4,
//         #[max_length = 100]
//         title -> Nullable<Varchar>,
//         #[max_length = 100]
//         image -> Nullable<Varchar>,
//         content -> Nullable<Text>,
//         #[max_length = 10]
//         slug -> Nullable<Varchar>,
//         active -> Bool,
//         order -> Int4,
//         created -> Nullable<Timestamptz>,
//         updated -> Nullable<Timestamptz>,
//         link -> Nullable<Text>,
//     }
// }

// diesel::table! {
//     api_category (id) {
//         id -> Int4,
//         #[max_length = 50]
//         name -> Varchar,
//         lft -> Int4,
//         rght -> Int4,
//         tree_id -> Int4,
//         level -> Int4,
//         parent_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     api_configuration (id) {
//         id -> Int4,
//         created_time -> Nullable<Timestamptz>,
//         updated_time -> Nullable<Timestamptz>,
//         #[max_length = 100]
//         name -> Nullable<Varchar>,
//         #[max_length = 1000]
//         description -> Nullable<Varchar>,
//         #[max_length = 50]
//         key -> Varchar,
//         #[max_length = 500]
//         value -> Varchar,
//     }
// }

// diesel::table! {
//     api_countryimages (id) {
//         id -> Int4,
//         #[max_length = 500]
//         image0 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image1 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image2 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image3 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image4 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image5 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image6 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image7 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image8 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image9 -> Nullable<Varchar>,
//         #[max_length = 500]
//         image10 -> Nullable<Varchar>,
//         #[max_length = 5]
//         country_iso_code -> Nullable<Varchar>,
//         #[max_length = 50]
//         language -> Varchar,
//         #[max_length = 50]
//         country_name -> Nullable<Varchar>,
//         #[max_length = 50]
//         city -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     api_crawlingpost (id) {
//         id -> Int4,
//         #[max_length = 300]
//         source -> Nullable<Varchar>,
//         #[max_length = 500]
//         title -> Varchar,
//         #[max_length = 50]
//         post_type -> Nullable<Varchar>,
//         #[max_length = 500]
//         post_link -> Nullable<Varchar>,
//         post_date -> Nullable<Timestamptz>,
//         #[max_length = 200]
//         post_date_str -> Nullable<Varchar>,
//         #[max_length = 1000]
//         img_url -> Nullable<Varchar>,
//         #[max_length = 100]
//         country_name -> Nullable<Varchar>,
//         #[max_length = 10]
//         country_code -> Nullable<Varchar>,
//         #[max_length = 5]
//         country_iso_code -> Nullable<Varchar>,
//         #[max_length = 50]
//         language -> Varchar,
//         active -> Bool,
//         created -> Nullable<Timestamptz>,
//         updated -> Nullable<Timestamptz>,
//         order -> Int4,
//         article_length -> Nullable<Int4>,
//         #[max_length = 50]
//         city -> Nullable<Varchar>,
//         description -> Nullable<Text>,
//         images -> Array<Nullable<Varchar>>,
//         #[max_length = 255]
//         source_name -> Nullable<Varchar>,
//         #[max_length = 255]
//         writing_style -> Nullable<Varchar>,
//         #[max_length = 255]
//         travel_style -> Nullable<Varchar>,
//         category_id -> Nullable<Int4>,
//         state_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     api_crawlingpost_pin (id) {
//         id -> Int4,
//         crawlingpost_id -> Int4,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     api_customservice (id) {
//         id -> Int4,
//         #[max_length = 500]
//         title -> Nullable<Varchar>,
//         content -> Text,
//     }
// }

// diesel::table! {
//     api_hotel (id) {
//         id -> Int4,
//         delete_flag -> Bool,
//         #[max_length = 100]
//         address -> Nullable<Varchar>,
//         longitude -> Numeric,
//         latitude -> Numeric,
//         #[max_length = 100]
//         hotel_name -> Varchar,
//         #[max_length = 100]
//         email -> Varchar,
//         #[max_length = 100]
//         phone_number -> Varchar,
//         description -> Text,
//         created_time -> Timestamp,
//         #[max_length = 500]
//         img_url -> Nullable<Varchar>,
//         user_id -> Nullable<Int4>,
//         credit -> Nullable<Numeric>,
//         #[max_length = 100]
//         city -> Nullable<Varchar>,
//         #[max_length = 100]
//         country -> Nullable<Varchar>,
//         #[max_length = 10]
//         country_code -> Nullable<Varchar>,
//         #[max_length = 100]
//         district -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     api_hotelservice (id) {
//         id -> Int4,
//         #[max_length = 100]
//         service_name -> Varchar,
//         created_time -> Timestamp,
//         delete_flag -> Bool,
//         hotel_id -> Int4,
//         bookable -> Bool,
//         service_description -> Text,
//     }
// }

// diesel::table! {
//     api_hotelserviceasset (id) {
//         id -> Int4,
//         #[max_length = 500]
//         url -> Varchar,
//         created_time -> Timestamp,
//         delete_flag -> Bool,
//         hotel_service_id -> Int4,
//     }
// }

// diesel::table! {
//     api_hotelservicerequest (id) {
//         id -> Int4,
//         #[max_length = 100]
//         title -> Varchar,
//         #[max_length = 500]
//         note -> Nullable<Varchar>,
//         created_time -> Timestamp,
//         #[max_length = 20]
//         status -> Varchar,
//         hotel_service_id -> Int4,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     api_prices (id) {
//         id -> Int4,
//         #[max_length = 100]
//         name -> Varchar,
//         price -> Float8,
//         data -> Float8,
//         duration -> Int4,
//         #[max_length = 10]
//         currency -> Varchar,
//         #[max_length = 100]
//         banner -> Nullable<Varchar>,
//         base_price -> Nullable<Float8>,
//         #[max_length = 2]
//         country -> Nullable<Varchar>,
//         created_time -> Nullable<Timestamptz>,
//         height -> Nullable<Int4>,
//         order -> Int4,
//         #[max_length = 50]
//         plan_type -> Nullable<Varchar>,
//         updated_time -> Nullable<Timestamptz>,
//         width -> Nullable<Int4>,
//         additional_credit -> Nullable<Float8>,
//     }
// }

// diesel::table! {
//     api_referencecost (id) {
//         id -> Int4,
//         #[max_length = 255]
//         reference -> Varchar,
//         cost -> Numeric,
//         #[max_length = 200]
//         unit -> Varchar,
//         #[max_length = 255]
//         currency_id -> Nullable<Varchar>,
//         #[max_length = 5]
//         country_iso_code -> Nullable<Varchar>,
//         #[max_length = 50]
//         city -> Nullable<Varchar>,
//         #[max_length = 50]
//         country_name -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     api_sim (id) {
//         id -> Int4,
//         #[max_length = 100]
//         sim_id -> Varchar,
//         #[max_length = 100]
//         sim_number -> Varchar,
//         sim_balance -> Nullable<Float8>,
//         created_time -> Timestamp,
//         expired_date -> Nullable<Timestamp>,
//         subscriber_id -> Nullable<Int8>,
//         #[max_length = 50]
//         sim_serial -> Nullable<Varchar>,
//         #[max_length = 20]
//         status -> Nullable<Varchar>,
//         hotel_id -> Nullable<Int4>,
//         user_id -> Nullable<Int4>,
//         last_topup -> Nullable<Numeric>,
//         base_sim_balance -> Nullable<Float8>,
//         active -> Bool,
//         last_active -> Nullable<Timestamptz>,
//         esim -> Bool,
//         #[max_length = 255]
//         provider -> Varchar,
//         #[max_length = 255]
//         qr_code -> Nullable<Varchar>,
//         #[max_length = 50]
//         sim_number_origin -> Nullable<Varchar>,
//         fup_reset_date -> Nullable<Timestamptz>,
//         sim_active_date -> Nullable<Timestamptz>,
//         subscribed -> Bool,
//         use_fup_code -> Bool,
//         #[max_length = 50]
//         pin -> Nullable<Varchar>,
//         sent_email -> Bool,
//     }
// }

// diesel::table! {
//     api_simidmapper (id) {
//         id -> Int4,
//         #[max_length = 30]
//         imsi -> Varchar,
//         #[max_length = 30]
//         iccid -> Varchar,
//         hotel_id -> Nullable<Int4>,
//         esim -> Bool,
//         #[max_length = 255]
//         provider -> Varchar,
//         #[max_length = 255]
//         qr_code -> Nullable<Varchar>,
//         synced -> Bool,
//         #[max_length = 254]
//         last_email -> Nullable<Varchar>,
//         #[max_length = 30]
//         msisdn -> Nullable<Varchar>,
//         active -> Bool,
//         created -> Nullable<Timestamptz>,
//         #[max_length = 255]
//         booking_id -> Nullable<Varchar>,
//         updated -> Timestamptz,
//         assigned -> Bool,
//         #[max_length = 255]
//         joytel_pin -> Varchar,
//         sale_partner_id -> Nullable<Int4>,
//         sent_date -> Nullable<Timestamptz>,
//     }
// }

// diesel::table! {
//     api_simidmapper_products (id) {
//         id -> Int4,
//         simidmapper_id -> Int4,
//         product_id -> Int4,
//     }
// }

// diesel::table! {
//     api_state (id) {
//         id -> Int4,
//         #[max_length = 50]
//         name -> Varchar,
//         lft -> Int4,
//         rght -> Int4,
//         tree_id -> Int4,
//         level -> Int4,
//         parent_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     api_topuptransaction (id) {
//         id -> Int4,
//         created_time -> Nullable<Timestamp>,
//         pricing_plan_id -> Nullable<Int4>,
//         sim_id -> Int4,
//         user_id -> Nullable<Int4>,
//         updated_time -> Nullable<Timestamptz>,
//         amount -> Numeric,
//         #[max_length = 255]
//         source -> Varchar,
//     }
// }

// diesel::table! {
//     api_topuptransactionbyapi (id) {
//         id -> Int4,
//         created_time -> Nullable<Timestamptz>,
//         updated_time -> Nullable<Timestamptz>,
//         amount -> Numeric,
//         delete_flag -> Bool,
//         sim_id -> Int4,
//         application_id -> Nullable<Int4>,
//         package_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     api_topuptransactionbyhotel (id) {
//         id -> Int4,
//         created_time -> Timestamp,
//         hotel_id -> Nullable<Int4>,
//         topup_transaction_id -> Nullable<Int4>,
//         user_id -> Int4,
//         amount -> Numeric,
//         package_id -> Nullable<Int4>,
//         sim_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     api_useractivity (id) {
//         id -> Int4,
//         created_time -> Nullable<Timestamptz>,
//         updated_time -> Nullable<Timestamptz>,
//         #[max_length = 100]
//         activity_name -> Varchar,
//         #[max_length = 500]
//         description -> Nullable<Varchar>,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     api_vhcategory (id) {
//         id -> Int4,
//         #[max_length = 255]
//         name -> Varchar,
//         descriptions -> Text,
//     }
// }

// diesel::table! {
//     api_virtualholiday (id) {
//         id -> Int4,
//         #[max_length = 255]
//         title -> Varchar,
//         #[max_length = 255]
//         location -> Varchar,
//         img_url -> Text,
//         link -> Text,
//         post_date -> Timestamptz,
//         live -> Bool,
//         category_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     auth_group (id) {
//         id -> Int4,
//         #[max_length = 150]
//         name -> Varchar,
//     }
// }

// diesel::table! {
//     auth_group_permissions (id) {
//         id -> Int4,
//         group_id -> Int4,
//         permission_id -> Int4,
//     }
// }

// diesel::table! {
//     auth_permission (id) {
//         id -> Int4,
//         content_type_id -> Int4,
//         #[max_length = 100]
//         codename -> Varchar,
//         #[max_length = 255]
//         name -> Varchar,
//     }
// }

// diesel::table! {
//     auth_user (id) {
//         id -> Int4,
//         #[max_length = 128]
//         password -> Varchar,
//         last_login -> Nullable<Timestamp>,
//         is_superuser -> Bool,
//         #[max_length = 150]
//         username -> Varchar,
//         #[max_length = 150]
//         first_name -> Varchar,
//         #[max_length = 254]
//         email -> Varchar,
//         is_staff -> Bool,
//         is_active -> Bool,
//         date_joined -> Timestamp,
//         #[max_length = 150]
//         last_name -> Varchar,
//     }
// }

// diesel::table! {
//     auth_user_groups (id) {
//         id -> Int4,
//         user_id -> Int4,
//         group_id -> Int4,
//     }
// }

// diesel::table! {
//     auth_user_user_permissions (id) {
//         id -> Int4,
//         user_id -> Int4,
//         permission_id -> Int4,
//     }
// }

// diesel::table! {
//     authtoken_token (key) {
//         #[max_length = 40]
//         key -> Varchar,
//         created -> Timestamp,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     card_blacklist (id) {
//         id -> Int8,
//         #[max_length = 255]
//         name -> Nullable<Varchar>,
//         #[max_length = 255]
//         start_number -> Nullable<Varchar>,
//         #[max_length = 255]
//         end_number -> Nullable<Varchar>,
//         #[max_length = 255]
//         country -> Varchar,
//         #[max_length = 255]
//         country_code -> Varchar,
//         start_time -> Nullable<Time>,
//         end_time -> Nullable<Time>,
//     }
// }

// diesel::table! {
//     card_whitelist (id) {
//         id -> Int8,
//         #[max_length = 255]
//         name -> Nullable<Varchar>,
//         #[max_length = 255]
//         start_number -> Nullable<Varchar>,
//         #[max_length = 255]
//         end_number -> Nullable<Varchar>,
//         #[max_length = 255]
//         country -> Varchar,
//         #[max_length = 255]
//         country_code -> Varchar,
//         start_time -> Nullable<Time>,
//         end_time -> Nullable<Time>,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//     }
// }

// diesel::table! {
//     corsheaders_corsmodel (id) {
//         id -> Int4,
//         #[max_length = 255]
//         cors -> Varchar,
//     }
// }

// diesel::table! {
//     currency_cache (name) {
//         #[max_length = 3]
//         name -> Varchar,
//         rate -> Numeric,
//         update -> Timestamptz,
//         #[max_length = 2]
//         country -> Varchar,
//     }
// }

// diesel::table! {
//     custom_offer_creditpromotion (number) {
//         #[max_length = 6]
//         number -> Varchar,
//         created -> Timestamptz,
//         benefit_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     custom_offer_custombenefit (id) {
//         id -> Int4,
//         #[sql_name = "type"]
//         #[max_length = 128]
//         type_ -> Varchar,
//         value -> Numeric,
//         #[max_length = 255]
//         name -> Varchar,
//         product_types -> Array<Nullable<Varchar>>,
//     }
// }

// diesel::table! {
//     custom_offer_custombenefit_pricing_plans (id) {
//         id -> Int4,
//         custombenefit_id -> Int4,
//         prices_id -> Int4,
//     }
// }

// diesel::table! {
//     custom_offer_custombenefit_products (id) {
//         id -> Int4,
//         custombenefit_id -> Int4,
//         product_id -> Int4,
//     }
// }

// diesel::table! {
//     custom_offer_customvoucher (id) {
//         id -> Int4,
//         #[max_length = 128]
//         name -> Varchar,
//         #[max_length = 128]
//         code -> Varchar,
//         benefit_id -> Nullable<Int4>,
//         #[max_length = 255]
//         custommer -> Nullable<Varchar>,
//         enable -> Bool,
//         one_time -> Bool,
//     }
// }

// diesel::table! {
//     custom_offer_redeemlog (id) {
//         id -> Int4,
//         #[max_length = 255]
//         code -> Varchar,
//         #[max_length = 255]
//         email -> Nullable<Varchar>,
//         #[max_length = 255]
//         sim_serial -> Nullable<Varchar>,
//         created -> Nullable<Timestamptz>,
//         updated -> Nullable<Timestamptz>,
//         product_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     device_blacklist (id) {
//         id -> Int8,
//         device_id -> Text,
//         registration_id -> Text,
//         #[max_length = 64]
//         phone_type -> Nullable<Varchar>,
//         location -> Jsonb,
//     }
// }

// diesel::table! {
//     django_admin_log (id) {
//         id -> Int4,
//         object_id -> Nullable<Text>,
//         #[max_length = 200]
//         object_repr -> Varchar,
//         action_flag -> Int4,
//         change_message -> Text,
//         content_type_id -> Nullable<Int4>,
//         user_id -> Int4,
//         action_time -> Timestamp,
//     }
// }

// diesel::table! {
//     django_celery_beat_clockedschedule (id) {
//         id -> Int4,
//         clocked_time -> Timestamptz,
//     }
// }

// diesel::table! {
//     django_celery_beat_crontabschedule (id) {
//         id -> Int4,
//         #[max_length = 240]
//         minute -> Varchar,
//         #[max_length = 96]
//         hour -> Varchar,
//         #[max_length = 64]
//         day_of_week -> Varchar,
//         #[max_length = 124]
//         day_of_month -> Varchar,
//         #[max_length = 64]
//         month_of_year -> Varchar,
//         #[max_length = 63]
//         timezone -> Varchar,
//     }
// }

// diesel::table! {
//     django_celery_beat_intervalschedule (id) {
//         id -> Int4,
//         every -> Int4,
//         #[max_length = 24]
//         period -> Varchar,
//     }
// }

// diesel::table! {
//     django_celery_beat_periodictask (id) {
//         id -> Int4,
//         #[max_length = 200]
//         name -> Varchar,
//         #[max_length = 200]
//         task -> Varchar,
//         args -> Text,
//         kwargs -> Text,
//         #[max_length = 200]
//         queue -> Nullable<Varchar>,
//         #[max_length = 200]
//         exchange -> Nullable<Varchar>,
//         #[max_length = 200]
//         routing_key -> Nullable<Varchar>,
//         expires -> Nullable<Timestamptz>,
//         enabled -> Bool,
//         last_run_at -> Nullable<Timestamptz>,
//         total_run_count -> Int4,
//         date_changed -> Timestamptz,
//         description -> Text,
//         crontab_id -> Nullable<Int4>,
//         interval_id -> Nullable<Int4>,
//         solar_id -> Nullable<Int4>,
//         one_off -> Bool,
//         start_time -> Nullable<Timestamptz>,
//         priority -> Nullable<Int4>,
//         headers -> Text,
//         clocked_id -> Nullable<Int4>,
//         expire_seconds -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     django_celery_beat_periodictasks (ident) {
//         ident -> Int2,
//         last_update -> Timestamptz,
//     }
// }

// diesel::table! {
//     django_celery_beat_solarschedule (id) {
//         id -> Int4,
//         #[max_length = 24]
//         event -> Varchar,
//         latitude -> Numeric,
//         longitude -> Numeric,
//     }
// }

// diesel::table! {
//     django_content_type (id) {
//         id -> Int4,
//         #[max_length = 100]
//         app_label -> Varchar,
//         #[max_length = 100]
//         model -> Varchar,
//     }
// }

// diesel::table! {
//     django_flatpage (id) {
//         id -> Int4,
//         #[max_length = 100]
//         url -> Varchar,
//         #[max_length = 200]
//         title -> Varchar,
//         content -> Text,
//         enable_comments -> Bool,
//         #[max_length = 70]
//         template_name -> Varchar,
//         registration_required -> Bool,
//     }
// }

// diesel::table! {
//     django_flatpage_sites (id) {
//         id -> Int4,
//         flatpage_id -> Int4,
//         site_id -> Int4,
//     }
// }

// diesel::table! {
//     django_mailbox_mailbox (id) {
//         id -> Int4,
//         #[max_length = 255]
//         name -> Varchar,
//         #[max_length = 255]
//         uri -> Nullable<Varchar>,
//         #[max_length = 255]
//         from_email -> Nullable<Varchar>,
//         active -> Bool,
//         last_polling -> Nullable<Timestamptz>,
//     }
// }

// diesel::table! {
//     django_mailbox_message (id) {
//         id -> Int4,
//         #[max_length = 255]
//         subject -> Varchar,
//         #[max_length = 255]
//         message_id -> Varchar,
//         #[max_length = 255]
//         from_header -> Varchar,
//         to_header -> Text,
//         outgoing -> Bool,
//         body -> Text,
//         encoded -> Bool,
//         processed -> Timestamptz,
//         read -> Nullable<Timestamptz>,
//         in_reply_to_id -> Nullable<Int4>,
//         mailbox_id -> Int4,
//         #[max_length = 100]
//         eml -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     django_mailbox_messageattachment (id) {
//         id -> Int4,
//         headers -> Nullable<Text>,
//         #[max_length = 100]
//         document -> Varchar,
//         message_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     django_migrations (id) {
//         id -> Int4,
//         #[max_length = 255]
//         app -> Varchar,
//         #[max_length = 255]
//         name -> Varchar,
//         applied -> Timestamp,
//     }
// }

// diesel::table! {
//     django_session (session_key) {
//         #[max_length = 40]
//         session_key -> Varchar,
//         session_data -> Text,
//         expire_date -> Timestamp,
//     }
// }

// diesel::table! {
//     django_site (id) {
//         id -> Int4,
//         #[max_length = 50]
//         name -> Varchar,
//         #[max_length = 100]
//         domain -> Varchar,
//     }
// }

// diesel::table! {
//     email_message (id) {
//         id -> Int4,
//         #[max_length = 255]
//         subject -> Varchar,
//         body -> Text,
//         processed -> Timestamptz,
//         read -> Nullable<Timestamptz>,
//         done -> Bool,
//         #[max_length = 255]
//         source -> Varchar,
//         #[max_length = 255]
//         message_id -> Nullable<Varchar>,
//         json -> Nullable<Jsonb>,
//         created -> Nullable<Timestamptz>,
//     }
// }

// diesel::table! {
//     email_subscriber (id) {
//         id -> Int4,
//         #[max_length = 10]
//         category -> Varchar,
//         #[max_length = 254]
//         email -> Varchar,
//         created -> Timestamptz,
//     }
// }

// diesel::table! {
//     external_order (id) {
//         id -> Int4,
//         #[max_length = 255]
//         customer -> Varchar,
//         #[max_length = 255]
//         email -> Nullable<Varchar>,
//         #[max_length = 255]
//         booking_id -> Varchar,
//         start_date -> Nullable<Timestamptz>,
//         product_id -> Nullable<Int4>,
//         #[max_length = 255]
//         qr_code -> Nullable<Varchar>,
//         #[max_length = 255]
//         sim_id -> Nullable<Varchar>,
//         #[max_length = 255]
//         sim_serial -> Nullable<Varchar>,
//         booking_date -> Nullable<Timestamptz>,
//     }
// }

// diesel::table! {
//     fcm_django_fcmdevice (id) {
//         id -> Int4,
//         #[max_length = 255]
//         name -> Nullable<Varchar>,
//         active -> Bool,
//         date_created -> Nullable<Timestamptz>,
//         #[max_length = 255]
//         device_id -> Nullable<Varchar>,
//         registration_id -> Text,
//         #[sql_name = "type"]
//         #[max_length = 10]
//         type_ -> Varchar,
//         user_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     get_sim_rules (id) {
//         id -> Int4,
//         #[max_length = 255]
//         name -> Varchar,
//         #[max_length = 255]
//         rule -> Varchar,
//         #[max_length = 255]
//         value -> Varchar,
//     }
// }

// diesel::table! {
//     hotel_topup_history (id) {
//         id -> Int4,
//         delete_flag -> Bool,
//         amount -> Nullable<Numeric>,
//         created_time -> Timestamptz,
//         create_by_id -> Int4,
//         hotel_id -> Int4,
//     }
// }

// diesel::table! {
//     journal (id) {
//         id -> Int4,
//         date -> Nullable<Date>,
//         #[max_length = 255]
//         title -> Nullable<Varchar>,
//         description -> Nullable<Text>,
//         start -> Nullable<Date>,
//         end -> Nullable<Date>,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         #[max_length = 50]
//         category -> Nullable<Varchar>,
//         user_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     journal_category (name) {
//         #[max_length = 200]
//         name -> Varchar,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//     }
// }

// diesel::table! {
//     journal_images (id) {
//         id -> Int4,
//         #[max_length = 100]
//         image -> Nullable<Varchar>,
//         #[max_length = 250]
//         name -> Nullable<Varchar>,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         journal_id -> Nullable<Int4>,
//         activities_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     nation_seal (id) {
//         id -> Int4,
//         #[max_length = 255]
//         name -> Nullable<Varchar>,
//         #[max_length = 100]
//         image -> Nullable<Varchar>,
//         #[max_length = 255]
//         country_name -> Nullable<Varchar>,
//         #[max_length = 255]
//         country_iso_code -> Nullable<Varchar>,
//         #[max_length = 255]
//         city -> Nullable<Varchar>,
//         created -> Timestamptz,
//         #[max_length = 255]
//         state -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     nation_seal_user (id) {
//         id -> Int4,
//         nationseal_id -> Int4,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     notifications_promonotification (id) {
//         id -> Int4,
//         #[max_length = 100]
//         title -> Varchar,
//         #[max_length = 500]
//         message -> Varchar,
//         content -> Nullable<Text>,
//         created_time -> Timestamptz,
//         #[sql_name = "type"]
//         #[max_length = 20]
//         type_ -> Varchar,
//     }
// }

// diesel::table! {
//     notifications_promonotification_users (id) {
//         id -> Int4,
//         promonotification_id -> Int4,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     notifications_promonotificationreadby (id) {
//         id -> Int4,
//         created_time -> Timestamptz,
//         notification_id -> Int4,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     oauth2_provider_accesstoken (id) {
//         id -> Int8,
//         #[max_length = 255]
//         token -> Varchar,
//         expires -> Timestamptz,
//         scope -> Text,
//         application_id -> Nullable<Int8>,
//         user_id -> Nullable<Int4>,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         source_refresh_token_id -> Nullable<Int8>,
//         id_token_id -> Nullable<Int8>,
//     }
// }

// diesel::table! {
//     oauth2_provider_application (id) {
//         id -> Int8,
//         #[max_length = 100]
//         client_id -> Varchar,
//         redirect_uris -> Text,
//         #[max_length = 32]
//         client_type -> Varchar,
//         #[max_length = 32]
//         authorization_grant_type -> Varchar,
//         #[max_length = 255]
//         client_secret -> Varchar,
//         #[max_length = 255]
//         name -> Varchar,
//         user_id -> Nullable<Int4>,
//         skip_authorization -> Bool,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         #[max_length = 5]
//         algorithm -> Varchar,
//         post_logout_redirect_uris -> Text,
//     }
// }

// diesel::table! {
//     oauth2_provider_grant (id) {
//         id -> Int8,
//         #[max_length = 255]
//         code -> Varchar,
//         expires -> Timestamptz,
//         redirect_uri -> Text,
//         scope -> Text,
//         application_id -> Int8,
//         user_id -> Int4,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         #[max_length = 128]
//         code_challenge -> Varchar,
//         #[max_length = 10]
//         code_challenge_method -> Varchar,
//         #[max_length = 255]
//         nonce -> Varchar,
//         claims -> Text,
//     }
// }

// diesel::table! {
//     oauth2_provider_idtoken (id) {
//         id -> Int8,
//         jti -> Uuid,
//         expires -> Timestamptz,
//         scope -> Text,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         application_id -> Nullable<Int8>,
//         user_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     oauth2_provider_refreshtoken (id) {
//         id -> Int8,
//         #[max_length = 255]
//         token -> Varchar,
//         access_token_id -> Nullable<Int8>,
//         application_id -> Int8,
//         user_id -> Int4,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         revoked -> Nullable<Timestamptz>,
//     }
// }

// diesel::table! {
//     package_settings (id) {
//         id -> Int4,
//         #[max_length = 255]
//         country -> Varchar,
//         #[max_length = 255]
//         source -> Varchar,
//         mb -> Nullable<Int4>,
//         days -> Int4,
//         priority -> Int4,
//         product_id -> Int4,
//         call -> Bool,
//     }
// }

// diesel::table! {
//     partner_partnerproduct (id) {
//         id -> Int8,
//         #[max_length = 100]
//         name -> Varchar,
//         price -> Numeric,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         product_id -> Int4,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     partner_partnertransaction (id) {
//         id -> Int8,
//         amount -> Numeric,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         partner_product_id -> Nullable<Int8>,
//         user_id -> Int4,
//         #[max_length = 255]
//         email -> Nullable<Varchar>,
//         #[max_length = 255]
//         sim_serial -> Nullable<Varchar>,
//         esim -> Bool,
//     }
// }

// diesel::table! {
//     partner_request_esim_log (id) {
//         id -> Int8,
//         #[max_length = 255]
//         booking_id -> Varchar,
//         #[max_length = 255]
//         email -> Nullable<Varchar>,
//         #[max_length = 255]
//         name -> Nullable<Varchar>,
//         #[max_length = 255]
//         platform -> Nullable<Varchar>,
//         #[max_length = 255]
//         country -> Nullable<Varchar>,
//         success -> Bool,
//         error -> Varchar,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//     }
// }

// diesel::table! {
//     payment_intent_request (id) {
//         id -> Int8,
//         #[max_length = 255]
//         email -> Nullable<Varchar>,
//         #[max_length = 255]
//         name -> Nullable<Varchar>,
//         payment_id -> Text,
//         data -> Jsonb,
//         #[max_length = 255]
//         country -> Varchar,
//         #[max_length = 255]
//         country_code -> Varchar,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//     }
// }

// diesel::table! {
//     personalities (id) {
//         id -> Int4,
//         #[max_length = 255]
//         title -> Nullable<Varchar>,
//         value -> Nullable<Text>,
//         #[max_length = 255]
//         author -> Nullable<Varchar>,
//         image -> Nullable<Text>,
//         #[max_length = 255]
//         location -> Nullable<Varchar>,
//         created -> Timestamptz,
//         user_id -> Nullable<Int4>,
//         #[max_length = 255]
//         city -> Nullable<Varchar>,
//         #[max_length = 5]
//         country_iso_code -> Nullable<Varchar>,
//         #[max_length = 255]
//         country_name -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     point_transactions (id) {
//         id -> Int4,
//         #[max_length = 255]
//         transaction_id -> Nullable<Varchar>,
//         point -> Int4,
//         amount -> Int4,
//         products -> Array<Nullable<Varchar>>,
//         synced -> Bool,
//         created -> Timestamptz,
//         created_user_id -> Nullable<Int4>,
//         profile_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     post_card (id) {
//         id -> Int4,
//         #[max_length = 250]
//         text -> Nullable<Varchar>,
//         #[max_length = 100]
//         image -> Nullable<Varchar>,
//         stamp -> Nullable<Timestamptz>,
//         created -> Timestamptz,
//         from_user_id -> Nullable<Int4>,
//         #[max_length = 254]
//         to_email -> Nullable<Varchar>,
//         #[sql_name = "type"]
//         #[max_length = 50]
//         type_ -> Varchar,
//     }
// }

// diesel::table! {
//     post_card_frame (id) {
//         id -> Int4,
//         #[max_length = 250]
//         name -> Nullable<Varchar>,
//         #[max_length = 100]
//         image -> Nullable<Varchar>,
//         created -> Timestamptz,
//         active -> Bool,
//         #[max_length = 255]
//         city -> Nullable<Varchar>,
//         #[max_length = 255]
//         country_name -> Nullable<Varchar>,
//         #[max_length = 5]
//         country_iso_code -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     post_card_to_user (id) {
//         id -> Int4,
//         userpostcard_id -> Int4,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     rates (id) {
//         id -> Int4,
//         #[max_length = 64]
//         country -> Nullable<Varchar>,
//         #[max_length = 255]
//         code -> Nullable<Varchar>,
//         data -> Float8,
//         call -> Float8,
//         #[max_length = 255]
//         operator -> Nullable<Varchar>,
//         #[max_length = 64]
//         network_type -> Varchar,
//         active -> Bool,
//     }
// }

// diesel::table! {
//     redemptions (id) {
//         id -> Int4,
//         #[max_length = 255]
//         name -> Varchar,
//         required_point -> Int4,
//         valid_from -> Nullable<Timestamptz>,
//         valid_to -> Nullable<Timestamptz>,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//         program_type -> Varchar,
//     }
// }

// diesel::table! {
//     redemptions_products (id) {
//         id -> Int4,
//         redemptions_id -> Int4,
//         product_id -> Int4,
//     }
// }

// diesel::table! {
//     rest_framework_api_key_apikey (id) {
//         #[max_length = 150]
//         id -> Varchar,
//         created -> Timestamptz,
//         #[max_length = 50]
//         name -> Varchar,
//         revoked -> Bool,
//         expiry_date -> Nullable<Timestamptz>,
//         #[max_length = 150]
//         hashed_key -> Varchar,
//         #[max_length = 8]
//         prefix -> Varchar,
//     }
// }

// diesel::table! {
//     shop_module_cartitem (id) {
//         id -> Int4,
//         created -> Timestamptz,
//         modified -> Timestamptz,
//         quantity -> Int2,
//         price -> Numeric,
//         tax -> Numeric,
//         #[max_length = 250]
//         product_type -> Varchar,
//         #[max_length = 512]
//         name -> Varchar,
//         #[max_length = 512]
//         image -> Nullable<Varchar>,
//         expired -> Nullable<Timestamptz>,
//         order_id -> Int4,
//         package_id -> Nullable<Int4>,
//         #[max_length = 250]
//         coupon -> Nullable<Varchar>,
//         is_expired -> Bool,
//         #[max_length = 250]
//         sim_serial -> Nullable<Varchar>,
//         esim -> Bool,
//         #[max_length = 50]
//         sim_id -> Nullable<Varchar>,
//         start_date -> Nullable<Timestamptz>,
//         #[max_length = 10]
//         product_id -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     shop_module_currency (code) {
//         created -> Timestamptz,
//         modified -> Timestamptz,
//         #[max_length = 255]
//         code -> Varchar,
//         #[max_length = 16]
//         label -> Varchar,
//     }
// }

// diesel::table! {
//     shop_module_deliveryrate (id) {
//         id -> Int4,
//         #[max_length = 255]
//         product_type -> Nullable<Varchar>,
//         amount -> Float8,
//         #[max_length = 2]
//         country -> Varchar,
//         #[max_length = 255]
//         region -> Nullable<Varchar>,
//         #[max_length = 255]
//         currency_id -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     shop_module_fuppackage (id) {
//         id -> Int4,
//         #[max_length = 64]
//         usage_type -> Varchar,
//         code -> Varchar,
//         #[max_length = 64]
//         sms_code -> Nullable<Varchar>,
//         #[max_length = 64]
//         call_code -> Nullable<Varchar>,
//         #[max_length = 250]
//         name -> Varchar,
//         data_mb -> Int4,
//         data_mb2 -> Int4,
//         throtle1 -> Int4,
//         throtle2 -> Int4,
//         fup_code -> Varchar,
//         days -> Int4,
//         per_day -> Bool,
//         #[max_length = 64]
//         sim_rule -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     shop_module_joyteltransaction (id) {
//         id -> Int4,
//         #[max_length = 255]
//         transaction_id -> Varchar,
//         created -> Nullable<Timestamptz>,
//         updated -> Nullable<Timestamptz>,
//         #[max_length = 255]
//         api -> Varchar,
//         body -> Jsonb,
//         response -> Jsonb,
//         sale_partner_id -> Nullable<Int4>,
//         #[max_length = 64]
//         sim_serial -> Varchar,
//     }
// }

// diesel::table! {
//     shop_module_order (id) {
//         id -> Int4,
//         created -> Timestamptz,
//         modified -> Timestamptz,
//         delivery_address -> Nullable<Text>,
//         #[max_length = 512]
//         receiver_name -> Varchar,
//         #[max_length = 512]
//         receiver_phone -> Varchar,
//         #[max_length = 250]
//         sim_serial -> Nullable<Varchar>,
//         #[max_length = 250]
//         coupon -> Nullable<Varchar>,
//         customer_id -> Nullable<Int4>,
//         #[max_length = 255]
//         status_id -> Varchar,
//         #[max_length = 250]
//         stripe_token -> Nullable<Varchar>,
//         #[max_length = 512]
//         receiver_email -> Nullable<Varchar>,
//         esim -> Bool,
//         product_id -> Nullable<Int4>,
//         start_active_date -> Nullable<Timestamptz>,
//         amount -> Numeric,
//         base_amount -> Numeric,
//         #[max_length = 64]
//         source -> Varchar,
//         pricing_plan_id -> Nullable<Int4>,
//         body -> Jsonb,
//         #[max_length = 250]
//         order_id -> Nullable<Varchar>,
//         consummed -> Bool,
//         sale_partner_id -> Nullable<Int4>,
//         payment -> Jsonb,
//     }
// }

// diesel::table! {
//     shop_module_package (id) {
//         id -> Int4,
//         #[max_length = 250]
//         package_name -> Varchar,
//         days -> Int4,
//         package_code -> Varchar,
//         #[max_length = 100]
//         provider -> Varchar,
//         #[max_length = 200]
//         country -> Nullable<Varchar>,
//         esim_code -> Varchar,
//         per_day -> Bool,
//         #[max_length = 10]
//         mcc -> Nullable<Varchar>,
//         fup_id -> Nullable<Int4>,
//         mb -> Int4,
//     }
// }

// diesel::table! {
//     shop_module_product (id) {
//         id -> Int4,
//         created -> Timestamptz,
//         modified -> Timestamptz,
//         #[max_length = 250]
//         product_type -> Varchar,
//         #[max_length = 512]
//         name -> Varchar,
//         #[max_length = 512]
//         image -> Nullable<Varchar>,
//         in_stock -> Int4,
//         price -> Numeric,
//         tax -> Numeric,
//         days -> Int4,
//         #[max_length = 255]
//         currency_id -> Nullable<Varchar>,
//         package_id -> Nullable<Int4>,
//         #[max_length = 512]
//         protect_code -> Nullable<Varchar>,
//         active -> Bool,
//         #[max_length = 255]
//         provider -> Varchar,
//         #[max_length = 250]
//         special_type -> Varchar,
//         additional_credit -> Numeric,
//         fup_reset_in -> Int4,
//         is_free -> Bool,
//         countries -> Array<Nullable<Varchar>>,
//         popular -> Bool,
//         use_credit -> Bool,
//         #[max_length = 200]
//         country -> Nullable<Varchar>,
//         show_in_app -> Bool,
//         offset_time -> Int4,
//         buy_count -> Int4,
//         internal_popular -> Bool,
//         show_in_web -> Bool,
//         option_in_web -> Bool,
//         #[max_length = 20]
//         sku -> Varchar,
//         description -> Nullable<Text>,
//         mb -> Int4,
//         web_active -> Bool,
//         reviews -> Int4,
//         delivery_cost -> Numeric,
//         fup_id -> Nullable<Int4>,
//         per_day -> Bool,
//         rule_id -> Nullable<Int4>,
//         calculate_percent -> Float8,
//         stars -> Float8,
//         bonus_point -> Int4,
//         region_package -> Bool,
//         calculate_mb -> Int4,
//     }
// }

// diesel::table! {
//     shop_module_product_packages (id) {
//         id -> Int4,
//         product_id -> Int4,
//         package_id -> Int4,
//     }
// }

// diesel::table! {
//     shop_module_product_user (id) {
//         id -> Int4,
//         product_id -> Int4,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     shop_module_providerapilog (id) {
//         id -> Int4,
//         created -> Nullable<Timestamptz>,
//         updated -> Nullable<Timestamptz>,
//         #[max_length = 255]
//         api -> Varchar,
//         body -> Jsonb,
//         response -> Jsonb,
//         #[max_length = 255]
//         status -> Nullable<Varchar>,
//         #[max_length = 255]
//         provider -> Varchar,
//         #[max_length = 255]
//         sim -> Varchar,
//         #[max_length = 255]
//         transaction_id -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     shop_module_revieworder (booking_id) {
//         #[max_length = 255]
//         booking_id -> Varchar,
//         #[max_length = 255]
//         email -> Nullable<Varchar>,
//         #[max_length = 255]
//         name -> Nullable<Varchar>,
//         #[max_length = 255]
//         product -> Nullable<Varchar>,
//         other_info -> Nullable<Text>,
//     }
// }

// diesel::table! {
//     shop_module_salepartner (id) {
//         id -> Int4,
//         #[max_length = 255]
//         message_id -> Nullable<Varchar>,
//         created -> Timestamptz,
//         modified -> Timestamptz,
//         #[max_length = 255]
//         booking_id -> Varchar,
//         #[max_length = 255]
//         name -> Nullable<Varchar>,
//         #[max_length = 255]
//         email -> Nullable<Varchar>,
//         #[max_length = 255]
//         phone_number -> Nullable<Varchar>,
//         #[max_length = 255]
//         product -> Nullable<Varchar>,
//         start_date -> Nullable<Timestamptz>,
//         send -> Bool,
//         #[max_length = 200]
//         country -> Nullable<Varchar>,
//         #[max_length = 50]
//         source -> Nullable<Varchar>,
//         esim -> Bool,
//         sim_number -> Nullable<Text>,
//         #[max_length = 255]
//         sent_date -> Nullable<Varchar>,
//         #[max_length = 255]
//         address -> Nullable<Varchar>,
//         package_id -> Nullable<Int4>,
//         #[max_length = 255]
//         real_option -> Nullable<Varchar>,
//         #[max_length = 255]
//         real_product -> Nullable<Varchar>,
//         days -> Int4,
//         gb -> Float8,
//         email_message_id -> Nullable<Int4>,
//         #[max_length = 200]
//         from_country -> Nullable<Varchar>,
//         booking_date -> Nullable<Timestamptz>,
//         quantity -> Int4,
//         sent_sims -> Int4,
//         active -> Bool,
//         note -> Nullable<Text>,
//         cart_id -> Nullable<Int4>,
//         date_request -> Nullable<Timestamptz>,
//         order_data -> Jsonb,
//         #[max_length = 50]
//         status -> Varchar,
//         activated -> Bool,
//         uid -> Nullable<Uuid>,
//         user_id -> Nullable<Int4>,
//         #[max_length = 200]
//         buyer_country -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     shop_module_status (value) {
//         created -> Timestamptz,
//         modified -> Timestamptz,
//         #[max_length = 255]
//         value -> Varchar,
//         #[max_length = 512]
//         label -> Varchar,
//         default -> Bool,
//         order_index -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     shop_module_transaction (id) {
//         id -> Int4,
//         created -> Timestamptz,
//         amount -> Numeric,
//         detail -> Text,
//         #[max_length = 200]
//         stripe_token -> Nullable<Varchar>,
//         #[max_length = 200]
//         sim_serial -> Nullable<Varchar>,
//         #[max_length = 200]
//         coupon -> Nullable<Varchar>,
//         order_id -> Int4,
//         user_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     sim_package (id) {
//         id -> Int4,
//         start_date -> Nullable<Timestamptz>,
//         end_date -> Nullable<Timestamptz>,
//         #[max_length = 255]
//         package_name -> Varchar,
//         #[max_length = 255]
//         provider -> Varchar,
//         used_percent -> Float8,
//         product_id -> Nullable<Int4>,
//         sim_id -> Int4,
//         created -> Nullable<Timestamptz>,
//         date_used -> Nullable<Timestamptz>,
//         updated -> Nullable<Timestamptz>,
//         expired -> Bool,
//         #[max_length = 255]
//         serial_number -> Nullable<Varchar>,
//         #[max_length = 255]
//         source -> Varchar,
//         total -> Int4,
//         used -> Int4,
//         #[max_length = 255]
//         order_id -> Nullable<Varchar>,
//         #[max_length = 255]
//         recharge_code -> Nullable<Varchar>,
//         #[max_length = 255]
//         joytel_rsp_order_id -> Nullable<Varchar>,
//         #[max_length = 255]
//         booking_id -> Nullable<Varchar>,
//         sent -> Bool,
//     }
// }

// diesel::table! {
//     socialaccount_socialaccount (id) {
//         id -> Int4,
//         #[max_length = 200]
//         provider -> Varchar,
//         #[max_length = 191]
//         uid -> Varchar,
//         last_login -> Timestamp,
//         date_joined -> Timestamp,
//         user_id -> Int4,
//         extra_data -> Jsonb,
//     }
// }

// diesel::table! {
//     socialaccount_socialapp (id) {
//         id -> Int4,
//         #[max_length = 30]
//         provider -> Varchar,
//         #[max_length = 40]
//         name -> Varchar,
//         #[max_length = 191]
//         client_id -> Varchar,
//         #[max_length = 191]
//         key -> Varchar,
//         #[max_length = 191]
//         secret -> Varchar,
//         #[max_length = 200]
//         provider_id -> Varchar,
//         settings -> Jsonb,
//     }
// }

// diesel::table! {
//     socialaccount_socialapp_sites (id) {
//         id -> Int4,
//         socialapp_id -> Int4,
//         site_id -> Int4,
//     }
// }

// diesel::table! {
//     socialaccount_socialtoken (id) {
//         id -> Int4,
//         token -> Text,
//         token_secret -> Text,
//         expires_at -> Nullable<Timestamp>,
//         account_id -> Int4,
//         app_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     taggit_tag (id) {
//         id -> Int4,
//         #[max_length = 100]
//         name -> Varchar,
//         #[max_length = 100]
//         slug -> Varchar,
//     }
// }

// diesel::table! {
//     taggit_taggeditem (id) {
//         id -> Int4,
//         object_id -> Int4,
//         content_type_id -> Int4,
//         tag_id -> Int4,
//     }
// }

// diesel::table! {
//     top_essential (id) {
//         id -> Int4,
//         #[max_length = 255]
//         title -> Varchar,
//         #[max_length = 255]
//         category -> Nullable<Varchar>,
//         image -> Text,
//         url -> Text,
//         #[max_length = 255]
//         location -> Nullable<Varchar>,
//         created -> Timestamptz,
//         #[max_length = 255]
//         latitude -> Nullable<Varchar>,
//         #[max_length = 255]
//         longitude -> Nullable<Varchar>,
//         description -> Nullable<Text>,
//         images -> Array<Nullable<Varchar>>,
//         #[max_length = 255]
//         city -> Nullable<Varchar>,
//         #[max_length = 5]
//         country_iso_code -> Nullable<Varchar>,
//         #[max_length = 255]
//         country_name -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     translations (id) {
//         id -> Int8,
//         #[max_length = 255]
//         country -> Varchar,
//         data -> Jsonb,
//         active -> Bool,
//     }
// }

// diesel::table! {
//     user_application (id) {
//         id -> Int4,
//         #[max_length = 20]
//         scope -> Varchar,
//         #[max_length = 150]
//         api_key_id -> Varchar,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     user_application_profile (id) {
//         id -> Int4,
//         received -> Bool,
//         #[max_length = 100]
//         avatar -> Nullable<Varchar>,
//         age -> Nullable<Int4>,
//         #[max_length = 500]
//         address -> Nullable<Varchar>,
//         birthday -> Nullable<Timestamptz>,
//         #[max_length = 10]
//         gender -> Nullable<Varchar>,
//         #[max_length = 300]
//         social_id -> Nullable<Varchar>,
//         #[max_length = 100]
//         country -> Nullable<Varchar>,
//         #[max_length = 100]
//         language -> Nullable<Varchar>,
//         user_id -> Int4,
//         first_login -> Bool,
//         connected_sim_id -> Nullable<Int4>,
//         new_sim_id -> Nullable<Int4>,
//         accept_terms -> Bool,
//         allow_email -> Bool,
//         #[max_length = 255]
//         email -> Nullable<Varchar>,
//         #[max_length = 255]
//         membership_number -> Nullable<Varchar>,
//         #[max_length = 20]
//         phone_number -> Nullable<Varchar>,
//         point -> Int4,
//         #[max_length = 255]
//         tier -> Varchar,
//     }
// }

// diesel::table! {
//     user_balance (id) {
//         id -> Int8,
//         balance -> Numeric,
//         last_topup -> Numeric,
//         created -> Nullable<Timestamptz>,
//         updated -> Nullable<Timestamptz>,
//         user_id -> Int4,
//     }
// }

// diesel::table! {
//     user_blacklist (id) {
//         id -> Int8,
//         #[max_length = 64]
//         email -> Varchar,
//         #[max_length = 64]
//         name -> Nullable<Varchar>,
//         #[max_length = 64]
//         ip -> Nullable<Varchar>,
//         location -> Jsonb,
//     }
// }

// diesel::table! {
//     user_passport (id) {
//         id -> Int4,
//         #[max_length = 255]
//         title -> Nullable<Varchar>,
//         #[max_length = 255]
//         subtitle -> Nullable<Varchar>,
//         #[max_length = 100]
//         image -> Nullable<Varchar>,
//         created -> Timestamptz,
//         first_page -> Bool,
//         seal_count -> Int4,
//         user_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     visited_location (id) {
//         id -> Int4,
//         #[max_length = 255]
//         location -> Nullable<Varchar>,
//         #[max_length = 255]
//         country_name -> Nullable<Varchar>,
//         #[max_length = 255]
//         country_iso_code -> Nullable<Varchar>,
//         #[max_length = 255]
//         city -> Nullable<Varchar>,
//         #[max_length = 255]
//         latitude -> Nullable<Varchar>,
//         #[max_length = 255]
//         longitude -> Nullable<Varchar>,
//         sealed -> Bool,
//         created -> Timestamptz,
//         user_id -> Nullable<Int4>,
//     }
// }

// diesel::table! {
//     weather_cache (key) {
//         #[max_length = 255]
//         key -> Varchar,
//         data -> Jsonb,
//         next_day_data -> Jsonb,
//         update -> Timestamptz,
//         #[max_length = 100]
//         lat -> Nullable<Varchar>,
//         #[max_length = 100]
//         long -> Nullable<Varchar>,
//     }
// }

// diesel::table! {
//     x_chatbot_faq (id) {
//         id -> Int8,
//         question -> Text,
//         answer -> Text,
//         embedding -> Jsonb,
//         platform -> Text,
//     }
// }

// diesel::table! {
//     x_chatbot_olympicuser (id) {
//         id -> Int8,
//         email -> Text,
//         country -> Text,
//     }
// }

// diesel::table! {
//     zoho_email (request_id) {
//         #[max_length = 255]
//         email -> Varchar,
//         request_id -> Text,
//         #[max_length = 255]
//         booking_id -> Varchar,
//         #[max_length = 255]
//         uid -> Varchar,
//         created -> Timestamptz,
//         updated -> Timestamptz,
//     }
// }

// diesel::joinable!(activities -> auth_user (user_id));
// diesel::joinable!(activities -> journal (journal_id));
// diesel::joinable!(activities -> journal_category (category_id));
// diesel::joinable!(api_crawlingpost -> api_category (category_id));
// diesel::joinable!(api_crawlingpost -> api_state (state_id));
// diesel::joinable!(api_crawlingpost_pin -> api_crawlingpost (crawlingpost_id));
// diesel::joinable!(api_crawlingpost_pin -> auth_user (user_id));
// diesel::joinable!(api_hotel -> auth_user (user_id));
// diesel::joinable!(api_hotelservice -> api_hotel (hotel_id));
// diesel::joinable!(api_hotelserviceasset -> api_hotelservice (hotel_service_id));
// diesel::joinable!(api_hotelservicerequest -> api_hotelservice (hotel_service_id));
// diesel::joinable!(api_hotelservicerequest -> auth_user (user_id));
// diesel::joinable!(api_referencecost -> shop_module_currency (currency_id));
// diesel::joinable!(api_sim -> api_hotel (hotel_id));
// diesel::joinable!(api_sim -> auth_user (user_id));
// diesel::joinable!(api_simidmapper -> api_hotel (hotel_id));
// diesel::joinable!(api_simidmapper -> shop_module_salepartner (sale_partner_id));
// diesel::joinable!(api_simidmapper_products -> api_simidmapper (simidmapper_id));
// diesel::joinable!(api_simidmapper_products -> shop_module_product (product_id));
// diesel::joinable!(api_topuptransaction -> api_prices (pricing_plan_id));
// diesel::joinable!(api_topuptransaction -> api_sim (sim_id));
// diesel::joinable!(api_topuptransaction -> auth_user (user_id));
// diesel::joinable!(api_topuptransactionbyapi -> api_sim (sim_id));
// diesel::joinable!(api_topuptransactionbyapi -> shop_module_product (package_id));
// diesel::joinable!(api_topuptransactionbyapi -> user_application (application_id));
// diesel::joinable!(api_topuptransactionbyhotel -> api_hotel (hotel_id));
// diesel::joinable!(api_topuptransactionbyhotel -> api_sim (sim_id));
// diesel::joinable!(api_topuptransactionbyhotel -> api_topuptransaction (topup_transaction_id));
// diesel::joinable!(api_topuptransactionbyhotel -> auth_user (user_id));
// diesel::joinable!(api_topuptransactionbyhotel -> shop_module_product (package_id));
// diesel::joinable!(api_useractivity -> auth_user (user_id));
// diesel::joinable!(api_virtualholiday -> api_vhcategory (category_id));
// diesel::joinable!(auth_group_permissions -> auth_group (group_id));
// diesel::joinable!(auth_group_permissions -> auth_permission (permission_id));
// diesel::joinable!(auth_permission -> django_content_type (content_type_id));
// diesel::joinable!(auth_user_groups -> auth_group (group_id));
// diesel::joinable!(auth_user_groups -> auth_user (user_id));
// diesel::joinable!(auth_user_user_permissions -> auth_permission (permission_id));
// diesel::joinable!(auth_user_user_permissions -> auth_user (user_id));
// diesel::joinable!(authtoken_token -> auth_user (user_id));
// diesel::joinable!(custom_offer_creditpromotion -> custom_offer_custombenefit (benefit_id));
// diesel::joinable!(custom_offer_custombenefit_pricing_plans -> api_prices (prices_id));
// diesel::joinable!(custom_offer_custombenefit_pricing_plans -> custom_offer_custombenefit (custombenefit_id));
// diesel::joinable!(custom_offer_custombenefit_products -> custom_offer_custombenefit (custombenefit_id));
// diesel::joinable!(custom_offer_custombenefit_products -> shop_module_product (product_id));
// diesel::joinable!(custom_offer_customvoucher -> custom_offer_custombenefit (benefit_id));
// diesel::joinable!(custom_offer_redeemlog -> shop_module_product (product_id));
// diesel::joinable!(django_admin_log -> auth_user (user_id));
// diesel::joinable!(django_admin_log -> django_content_type (content_type_id));
// diesel::joinable!(django_celery_beat_periodictask -> django_celery_beat_clockedschedule (clocked_id));
// diesel::joinable!(django_celery_beat_periodictask -> django_celery_beat_crontabschedule (crontab_id));
// diesel::joinable!(django_celery_beat_periodictask -> django_celery_beat_intervalschedule (interval_id));
// diesel::joinable!(django_celery_beat_periodictask -> django_celery_beat_solarschedule (solar_id));
// diesel::joinable!(django_flatpage_sites -> django_flatpage (flatpage_id));
// diesel::joinable!(django_flatpage_sites -> django_site (site_id));
// diesel::joinable!(django_mailbox_message -> django_mailbox_mailbox (mailbox_id));
// diesel::joinable!(django_mailbox_messageattachment -> django_mailbox_message (message_id));
// diesel::joinable!(external_order -> shop_module_product (product_id));
// diesel::joinable!(fcm_django_fcmdevice -> auth_user (user_id));
// diesel::joinable!(hotel_topup_history -> api_hotel (hotel_id));
// diesel::joinable!(hotel_topup_history -> auth_user (create_by_id));
// diesel::joinable!(journal -> auth_user (user_id));
// diesel::joinable!(journal_images -> activities (activities_id));
// diesel::joinable!(journal_images -> journal (journal_id));
// diesel::joinable!(nation_seal_user -> auth_user (user_id));
// diesel::joinable!(nation_seal_user -> nation_seal (nationseal_id));
// diesel::joinable!(notifications_promonotification_users -> auth_user (user_id));
// diesel::joinable!(notifications_promonotification_users -> notifications_promonotification (promonotification_id));
// diesel::joinable!(notifications_promonotificationreadby -> auth_user (user_id));
// diesel::joinable!(notifications_promonotificationreadby -> notifications_promonotification (notification_id));
// diesel::joinable!(oauth2_provider_accesstoken -> auth_user (user_id));
// diesel::joinable!(oauth2_provider_accesstoken -> oauth2_provider_application (application_id));
// diesel::joinable!(oauth2_provider_accesstoken -> oauth2_provider_idtoken (id_token_id));
// diesel::joinable!(oauth2_provider_application -> auth_user (user_id));
// diesel::joinable!(oauth2_provider_grant -> auth_user (user_id));
// diesel::joinable!(oauth2_provider_grant -> oauth2_provider_application (application_id));
// diesel::joinable!(oauth2_provider_idtoken -> auth_user (user_id));
// diesel::joinable!(oauth2_provider_idtoken -> oauth2_provider_application (application_id));
// diesel::joinable!(oauth2_provider_refreshtoken -> auth_user (user_id));
// diesel::joinable!(oauth2_provider_refreshtoken -> oauth2_provider_application (application_id));
// diesel::joinable!(package_settings -> shop_module_product (product_id));
// diesel::joinable!(partner_partnerproduct -> auth_user (user_id));
// diesel::joinable!(partner_partnerproduct -> shop_module_product (product_id));
// diesel::joinable!(partner_partnertransaction -> auth_user (user_id));
// diesel::joinable!(partner_partnertransaction -> partner_partnerproduct (partner_product_id));
// diesel::joinable!(personalities -> auth_user (user_id));
// diesel::joinable!(point_transactions -> auth_user (created_user_id));
// diesel::joinable!(point_transactions -> user_application_profile (profile_id));
// diesel::joinable!(post_card -> auth_user (from_user_id));
// diesel::joinable!(post_card_to_user -> auth_user (user_id));
// diesel::joinable!(post_card_to_user -> post_card (userpostcard_id));
// diesel::joinable!(redemptions_products -> redemptions (redemptions_id));
// diesel::joinable!(redemptions_products -> shop_module_product (product_id));
// diesel::joinable!(shop_module_cartitem -> shop_module_order (order_id));
// diesel::joinable!(shop_module_cartitem -> shop_module_package (package_id));
// diesel::joinable!(shop_module_deliveryrate -> shop_module_currency (currency_id));
// diesel::joinable!(shop_module_joyteltransaction -> shop_module_salepartner (sale_partner_id));
// diesel::joinable!(shop_module_order -> api_prices (pricing_plan_id));
// diesel::joinable!(shop_module_order -> auth_user (customer_id));
// diesel::joinable!(shop_module_order -> shop_module_product (product_id));
// diesel::joinable!(shop_module_order -> shop_module_salepartner (sale_partner_id));
// diesel::joinable!(shop_module_order -> shop_module_status (status_id));
// diesel::joinable!(shop_module_package -> shop_module_fuppackage (fup_id));
// diesel::joinable!(shop_module_product -> get_sim_rules (rule_id));
// diesel::joinable!(shop_module_product -> shop_module_currency (currency_id));
// diesel::joinable!(shop_module_product -> shop_module_fuppackage (fup_id));
// diesel::joinable!(shop_module_product -> shop_module_package (package_id));
// diesel::joinable!(shop_module_product_packages -> shop_module_package (package_id));
// diesel::joinable!(shop_module_product_packages -> shop_module_product (product_id));
// diesel::joinable!(shop_module_product_user -> auth_user (user_id));
// diesel::joinable!(shop_module_product_user -> shop_module_product (product_id));
// diesel::joinable!(shop_module_salepartner -> auth_user (user_id));
// diesel::joinable!(shop_module_salepartner -> email_message (email_message_id));
// diesel::joinable!(shop_module_salepartner -> shop_module_cartitem (cart_id));
// diesel::joinable!(shop_module_salepartner -> shop_module_product (package_id));
// diesel::joinable!(shop_module_transaction -> auth_user (user_id));
// diesel::joinable!(shop_module_transaction -> shop_module_order (order_id));
// diesel::joinable!(sim_package -> api_sim (sim_id));
// diesel::joinable!(sim_package -> shop_module_product (product_id));
// diesel::joinable!(socialaccount_socialaccount -> auth_user (user_id));
// diesel::joinable!(socialaccount_socialapp_sites -> django_site (site_id));
// diesel::joinable!(socialaccount_socialapp_sites -> socialaccount_socialapp (socialapp_id));
// diesel::joinable!(socialaccount_socialtoken -> socialaccount_socialaccount (account_id));
// diesel::joinable!(socialaccount_socialtoken -> socialaccount_socialapp (app_id));
// diesel::joinable!(taggit_taggeditem -> django_content_type (content_type_id));
// diesel::joinable!(taggit_taggeditem -> taggit_tag (tag_id));
// diesel::joinable!(user_application -> auth_user (user_id));
// diesel::joinable!(user_application -> rest_framework_api_key_apikey (api_key_id));
// diesel::joinable!(user_application_profile -> auth_user (user_id));
// diesel::joinable!(user_balance -> auth_user (user_id));
// diesel::joinable!(user_passport -> auth_user (user_id));
// diesel::joinable!(visited_location -> auth_user (user_id));

// diesel::allow_tables_to_appear_in_same_query!(
//     activities,
//     api_advertisement,
//     api_category,
//     api_configuration,
//     api_countryimages,
//     api_crawlingpost,
//     api_crawlingpost_pin,
//     api_customservice,
//     api_hotel,
//     api_hotelservice,
//     api_hotelserviceasset,
//     api_hotelservicerequest,
//     api_prices,
//     api_referencecost,
//     api_sim,
//     api_simidmapper,
//     api_simidmapper_products,
//     api_state,
//     api_topuptransaction,
//     api_topuptransactionbyapi,
//     api_topuptransactionbyhotel,
//     api_useractivity,
//     api_vhcategory,
//     api_virtualholiday,
//     auth_group,
//     auth_group_permissions,
//     auth_permission,
//     auth_user,
//     auth_user_groups,
//     auth_user_user_permissions,
//     authtoken_token,
//     card_blacklist,
//     card_whitelist,
//     corsheaders_corsmodel,
//     currency_cache,
//     custom_offer_creditpromotion,
//     custom_offer_custombenefit,
//     custom_offer_custombenefit_pricing_plans,
//     custom_offer_custombenefit_products,
//     custom_offer_customvoucher,
//     custom_offer_redeemlog,
//     device_blacklist,
//     django_admin_log,
//     django_celery_beat_clockedschedule,
//     django_celery_beat_crontabschedule,
//     django_celery_beat_intervalschedule,
//     django_celery_beat_periodictask,
//     django_celery_beat_periodictasks,
//     django_celery_beat_solarschedule,
//     django_content_type,
//     django_flatpage,
//     django_flatpage_sites,
//     django_mailbox_mailbox,
//     django_mailbox_message,
//     django_mailbox_messageattachment,
//     django_migrations,
//     django_session,
//     django_site,
//     email_message,
//     email_subscriber,
//     external_order,
//     fcm_django_fcmdevice,
//     get_sim_rules,
//     hotel_topup_history,
//     journal,
//     journal_category,
//     journal_images,
//     nation_seal,
//     nation_seal_user,
//     notifications_promonotification,
//     notifications_promonotification_users,
//     notifications_promonotificationreadby,
//     oauth2_provider_accesstoken,
//     oauth2_provider_application,
//     oauth2_provider_grant,
//     oauth2_provider_idtoken,
//     oauth2_provider_refreshtoken,
//     package_settings,
//     partner_partnerproduct,
//     partner_partnertransaction,
//     partner_request_esim_log,
//     payment_intent_request,
//     personalities,
//     point_transactions,
//     post_card,
//     post_card_frame,
//     post_card_to_user,
//     rates,
//     redemptions,
//     redemptions_products,
//     rest_framework_api_key_apikey,
//     shop_module_cartitem,
//     shop_module_currency,
//     shop_module_deliveryrate,
//     shop_module_fuppackage,
//     shop_module_joyteltransaction,
//     shop_module_order,
//     shop_module_package,
//     shop_module_product,
//     shop_module_product_packages,
//     shop_module_product_user,
//     shop_module_providerapilog,
//     shop_module_revieworder,
//     shop_module_salepartner,
//     shop_module_status,
//     shop_module_transaction,
//     sim_package,
//     socialaccount_socialaccount,
//     socialaccount_socialapp,
//     socialaccount_socialapp_sites,
//     socialaccount_socialtoken,
//     taggit_tag,
//     taggit_taggeditem,
//     top_essential,
//     translations,
//     user_application,
//     user_application_profile,
//     user_balance,
//     user_blacklist,
//     user_passport,
//     visited_location,
//     weather_cache,
//     x_chatbot_faq,
//     x_chatbot_olympicuser,
//     zoho_email,
// );
